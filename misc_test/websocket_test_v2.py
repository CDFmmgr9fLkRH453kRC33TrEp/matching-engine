import asyncio
import websockets
import argparse
import json
import time




parser = argparse.ArgumentParser()

parser.add_argument('--ip', type=str, required=True)
parser.add_argument('--id', type=str, required=True)
parser.add_argument('--password', type=str, required=True)

args = parser.parse_args()

async def consumer(message):
    print("consumer triggered!")
    print(message)

async def consumer_handler(websocket):
    async for message in websocket:
        await consumer(message)

uri = f"ws://{args.ip}:4000/orders/ws"
print("uri:", uri)

async def producer():
    symbol = "AAPL"
    # price = int(m(math.sin(time.time()+off)))
    price = 1
    # side = random.choice(["Sell", "Buy"])
    side = "Buy"
    jsonreq = {
        'OrderType': side,
        'Amount': 1,
        'Price': price,
        'Symbol': symbol,
        'TraderId': f"{args.id}",
        'Password': list(args.password)
    }
    return json.dumps(jsonreq)

async def producer_handler(websocket):
    while True:
        message = await producer()
        await websocket.send(message)
        await asyncio.sleep(0.0001)


async def handler(websocket):
    consumer_task = asyncio.create_task(consumer_handler(websocket))
    producer_task = asyncio.create_task(producer_handler(websocket))
    done, pending = await asyncio.wait(
        [consumer_task, producer_task],
        return_when=asyncio.FIRST_COMPLETED,
    )
    for task in pending:
        task.cancel()

async def main():
    async with websockets.connect(uri) as websocket:
        await handler(websocket)

if __name__ == "__main__":
    asyncio.run(main())