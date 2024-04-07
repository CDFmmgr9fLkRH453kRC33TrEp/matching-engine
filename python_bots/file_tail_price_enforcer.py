import asyncio
import os
import websockets
from watchfiles import awatch
import json
import argparse
import numpy as np

parser = argparse.ArgumentParser()

parser.add_argument('--symbol', type=str, required=True)
parser.add_argument('--amt', type=str, required=True)

args = parser.parse_args()

max_price = 1000

# todo: implement flat, normal, bimodal, delta, smile and see what is the most fun
def generate_gaussian(min, max, mean, var):
    out_arr =  np.arange(min + 1, max - 1)
    # norm_pdf = lambda index: 1/((2 * 3.1415 * var)**0.5) * np.exp(-0.5 *)
    return 


async def tail_file_and_send_message(file_path, websocket_uri):
    async def send_message(price):
        # because we dont allow shorts (or do so via inversed products)
        # I dont think we need sell orders for price enforcement?
        print("sending buy order at price")
        jsonreq = {
            'OrderType': "Buy",
            'Amount': args.amt,
            'Price': price,
            'Symbol': args.symbol,
            'TraderId': "Price_Enforcer",
            'Password': list("penf")
        }
        await websocket.send(json.dumps(jsonreq))

    try:
        # Establish WebSocket connection
        async with websockets.connect(websocket_uri) as ws:
            # Watch for changes in the file
            async for changes in awatch(file_path):
                for action, _ in changes:
                    print("action", action)
                    with open(file_path, "r") as file:
                        new_line = file.readlines()[-1].strip()
                        print(new_line)
                        await send_message(new_line)

    except KeyboardInterrupt:
        pass

if __name__ == "__main__":
    # Replace "ws://your_websocket_server" with the actual WebSocket server URI
    websocket_uri = "ws://127.16.123.0:4000/orders/ws"
    
    # Replace "path/to/your/file.log" with the actual path to the file you want to tail
    file_path = "/Users/caidan/projects/exchange_simulator/matching-engine/python_bots/file.log"

    asyncio.run(tail_file_and_send_message(file_path, websocket_uri))
