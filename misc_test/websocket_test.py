import time
import websocket
import json
import argparse
import random
import schedule
import math

from scipy.interpolate import interp1d

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
    'Price': 9,
    'Symbol': "AAPL",
    'TraderId': f"{args.id}",
}

# ws.send(json.dumps(jsonreq))
# ws.send("Hello")
# time.sleep(20)

m = interp1d([-1, 1], [1, 10])
off = random.randint(-3,3)

def trade_rand():
    global flip
    symbol = random.choice(["AAPL", "JNJ"])
    price = int(m(math.sin(time.time()+off)))
    side = random.choice(["Sell", "Buy"])
    # if flip:
    #     flip = False
    #     side="Buy"
    # else:
    #     side="Sell"
    #     flip = True
    amount = random.randint(1, 10)
    jsonreq = {
        'OrderType': side,
        'Amount': 1,
        'Price': price,
        'Symbol': symbol,
        'TraderId': f"{args.id}",
    }

    ws.send(json.dumps(jsonreq))

# schedule.every(1).seconds.do(trade_rand)
# flip = False
# for i in range(1000):
#     trade_rand()
flip = False
while True:
    trade_rand()
    time.sleep(0.5)
    # print(ws.recv())
