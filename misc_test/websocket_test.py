import time
import websocket
import json

ws = websocket.WebSocket()

ws.connect("ws://localhost:3000/orders/ws")

jsonreq = {
            'OrderType': "Buy",
            'Amount': 3,
            'Price': 3,
            'Symbol': "AAPL",
            'TraderId': "Columbia_A",            
        }

ws.send(json.dumps(jsonreq))
# ws.send("Hello")
# time.sleep(20)
print(ws.recv())
