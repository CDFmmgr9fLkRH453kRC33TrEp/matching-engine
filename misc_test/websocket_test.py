import time
import websocket
import json
import argparse

# NEED TO SET UP LOOPBACK FOR TESTING
# sudo ifconfig lo0 alias 127.16.123.1 (columbia a)
# sudo ifconfig lo0 alias 127.16.123.2 (columbia b)

parser = argparse.ArgumentParser()

parser.add_argument('--ip', type=str, required=True)
parser.add_argument('--id', type=str, required=True)

args = parser.parse_args()

ws = websocket.WebSocket()

ws.connect(f"ws://{args.ip}:4000/orders/ws")

jsonreq = {
            'OrderType': "Buy",
            'Amount': 1,
            'Price': 4,
            'Symbol': "AAPL",
            'TraderId': f"{args.id}",            
        }

ws.send(json.dumps(jsonreq))
# ws.send("Hello")
# time.sleep(20)
while True:
    print(ws.recv())
