# Client to Server
## Limit Order Request Message
'OrderType': "Buy" | "Sell",
'Amount': int,
'Price': int,
'Symbol': string,
'TraderId': string,
'Password': 4 char array

## Cancel Order Request Message
'OrderID': string,
'Price': int,
'Symbol': string,
'Side': "Buy" | "Sell"
'Password': 4 char array


# Server to Client
## Limit Order Response Message

## Cancel Order Response Message

## Orderbook Update Message