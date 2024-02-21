import asyncio
import os
import websockets
from watchfiles import awatch
import json


async def tail_file_and_send_message(file_path, websocket_uri):
    async def send_message(price):
        print(price)
        jsonreq = {
            'OrderType': "Buy",
            'Amount': 1,
            'Price': price,
            'Symbol': "AAPL",
            'TraderId': "Columbia_A",
            'Password': list("cu_a")
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
