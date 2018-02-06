#!/usr/bin/python

import time
import zmq
import uuid
import yaml
import docker
from threading import Thread, Lock
from urllib.request import urlopen

import depot_pb2 as depot

HEARTBEAT_LIVENESS = 3
HEARTBEAT_INTERVAL = 1000

CURRENT_EP_NUM = 0
CURRENT_MAX_EP_NUM = 0
CURRENT_CONFIG = None


# Spawn and manage Docker container running experiment.
def start_docker():
    global CURRENT_CONFIG

    print("Spawning Docker container")

    client = docker.from_env(version="auto")
    container = client.containers.run("cannon/testing", detach=True,
                                      volumes={'test': {'bind': '/test', 'mode': 'rw'}},
                                      ports={5555: 5555})

    time.sleep(20)

    # Previous way of executing experiment runner; could be a better way to do this.
    (code, exec_stream) = container.exec_run('/bin/bash -c "source /home/cannon/rl_wksp/devel/setup.bash; '
                                  'python /home/cannon/reinforcement_learning/rl_agents/python/experiment_runner.py"',
                                  stream=True)

    for line in exec_stream:
        print(line)

    container.kill()
    time.sleep(20)


# Function to do work by spinning up Docker container and talking to it via ZMQ.
def do_work(num_eps):
    global CURRENT_EP_NUM
    global CURRENT_CONFIG

    with open("/var/lib/docker/volumes/test/_data/config.yaml", "w") as config_file:
        config_file.write(CURRENT_CONFIG.body)

    # Start Docker container in new thread
    t = Thread(target=start_docker)
    t.start()

    context = zmq.Context(1)
    receiver = context.socket(zmq.PULL)
    receiver.connect("tcp://localhost:5555")

    for i in range(num_eps):
        string = receiver.recv_string()
        print("Received {} on iteration {}".format(string, i))
        CURRENT_EP_NUM = int(string)
        time.sleep(0.01)


# Send report message
def send_report(socket, identity):
    global CURRENT_EP_NUM
    global CURRENT_CONFIG
    global CURRENT_MAX_EP_NUM

    report_msg = []
    report_msg.append("".encode())

    type_part = depot.TypeSignifier()
    type_part.type = depot.REPORT
    report_msg.append(type_part.SerializeToString())

    report_part = depot.ServerReport()
    config_done = False
    report_part.server_uuid = str(identity)
    report_part.ep_num = CURRENT_EP_NUM
    if CURRENT_CONFIG:
        report_part.has_config = True
        report_part.config_uuid = str(CURRENT_CONFIG.uuid)
        if CURRENT_MAX_EP_NUM - 1 == CURRENT_EP_NUM:
            report_part.done = True
            config_done = True
        else:
            report_part.done = False
    else:
        report_part.has_config = False

    if config_done:
        CURRENT_CONFIG = None
        CURRENT_EP_NUM = 0

    report_msg.append(report_part.SerializeToString())
    socket.send_multipart(report_msg)

def main():
    global CURRENT_CONFIG
    global CURRENT_EP_NUM
    global CURRENT_MAX_EP_NUM

    context = zmq.Context()
    identity = uuid.uuid4()
    poller = zmq.Poller()

    receiver = context.socket(zmq.DEALER)
    receiver.setsockopt(zmq.IDENTITY, identity.bytes)
    receiver.connect("tcp://rrl-exp.duckdns.org:5557")

    statistics = context.socket(zmq.PUB)
    statistics.setsockopt(zmq.IDENTITY, identity.bytes)
    statistics.connect("tcp://rrl-exp.duckdns.org:5558")

    print("I: Starting worker {} ({})".format(identity, identity.bytes))

    # Get public(ish) ip
    ip_string = str(urlopen('http://ip.42.pl/raw').read())

    init_msg = []
    init_msg.append("".encode())

    type_part = depot.TypeSignifier()
    type_part.type = depot.INIT
    init_msg.append(type_part.SerializeToString())

    init = depot.ServerInit()
    init.name = ip_string  # TODO: Take in name as parameter
    init.ip = ip_string
    init_msg.append(init.SerializeToString())

    receiver.send_multipart(init_msg)

    time.sleep(0.5)

    poller.register(receiver, zmq.POLLIN)

    while True:
        socks = dict(poller.poll(HEARTBEAT_INTERVAL))

        if socks.get(receiver) == zmq.POLLIN:
            msg = receiver.recv_multipart()
            print("D: received message {}".format(msg))
            type_part = depot.TypeSignifier()
            type_part.ParseFromString(msg[1])

            if type_part.type != depot.CONFIG:
                print("E: found unexpected message type {}".format(type_part.type))

                report_msg = []
                report_msg.append("".encode())
                type_part = depot.TypeSignifier()
                type_part.type = depot.REPORT
                report_msg.append(type_part.SerializeToString())
                report_msg.append("".encode())
                statistics.send_multipart(report_msg)

                return

            config_msg = depot.ServerConfig()
            config_msg.ParseFromString(msg[2])
            if not CURRENT_CONFIG:
                CURRENT_CONFIG = config_msg
                CURRENT_EP_NUM = 0
            else:
                print("E: Already have config!")
                return

            print("D: Got config message with name {}".format(config_msg.name))

            try:
                config = yaml.load(config_msg.body)
                print("D: Parsed YAML config body: {}".format(config))

                if "experiment" in config:
                    if "num_episodes" in config["experiment"]:
                        num_eps = config["experiment"]["num_episodes"]
                        print("D: Got config with {} episodes".format(num_eps))
                        CURRENT_MAX_EP_NUM = num_eps
                        t = Thread(target=do_work, args=(num_eps,))
                        t.start()
                    else:
                        print("E: Config didn't contain expected fields")
                else:
                    print("E: Config didn't contain expected fields")

            except yaml.YAMLError:
                print("E: Received message with non-YAML config body")

        send_report(statistics, identity)


if __name__ == "__main__":
    main()
