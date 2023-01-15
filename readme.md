# Rust Matching Engine, Limit Order Book, and Exchange Simulator
Simplistic exchange simulator with in memory order book and multi asset credit limit enforcement.

## Limit Order Book

## Matching Engine

## Exchange API
One connection/thread per trader, handles listening, reporting, and interfacing with matching engine. 
Handled via tokio and hyper
On request:
* Checks non malformed trade
* Checks legality of trade (i.e. respects credit limits, ratelimiting, etc.)
* Makes a call to handle incoming request of the appropriate orderbook struct. 
* This is blocking, and other connection handlers must wait for orderbook to be unlocked in order to pass a request
* Could implement some kind of queue here to allow temporary burst traffic, but I think the simple version should be enough

## Enforcing Credit Limits

## Account Management

## Resources/Further Reading
(Generally tried to sort from most to least useful)

https://web.archive.org/web/20110219163448/http://howtohft.wordpress.com/2011/02/15/how-to-build-a-fast-limit-order-book/

https://web.archive.org/web/20110219163418/http://howtohft.wordpress.com/2011/02/15/building-a-trading-system-general-considerations/

https://www.youtube.com/watch?v=b1e4t2k2KJY

https://www.sciencedirect.com/science/article/pii/S2352711022000875

https://sanket.tech/posts/rustbook/

https://docs.rs/orderbook/latest/orderbook/

https://devexperts.com/blog/what-it-takes-to-build-reliable-and-fast-exchange/

https://github.com/charles-cooper/itch-order-book

https://www.chrisstucchio.com/blog/2012/hft_apology.html

https://marabos.nl/atomics/basics.html

https://markrbest.github.io/hft-and-rust/

https://rustrepo.com/repo/uinb-galois

https://github.com/yangfh2004/rust-limit-order-book