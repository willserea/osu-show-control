from pythonosc.dispatcher import Dispatcher
from pythonosc.osc_server import BlockingOSCUDPServer


def print_handler(address, *args):
    print(f"{address}: {args}")


dispatcher = Dispatcher()
dispatcher.map("/*", print_handler)

server = BlockingOSCUDPServer(("127.0.0.1", 9000), dispatcher)
print("OSC Receiver läuft auf 127.0.0.1:9000")
server.serve_forever()
