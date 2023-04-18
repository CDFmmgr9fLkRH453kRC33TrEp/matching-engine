import ws from 'k6/ws';
import { check } from 'k6';

// maxes out around 10.7k/s

export let options = {
  scenarios: {
    trader1: {
      exec: 'buy_order',
      executor: 'constant-vus',
      vus: 1,
      duration: '3s',
      gracefulStop: '2s',
      env: {
        URL: 'ws://127.16.123.1:4000/orders/ws',
        TRADERID: 'Columbia_A',
      }
    },
    trader2: {
      exec: 'buy_order',
      executor: 'constant-vus',
      vus: 1,
      duration: '3s',
      gracefulStop: '2s',
      env: {
        URL: 'ws://127.16.123.2:4000/orders/ws',
        TRADERID: 'Columbia_B',
      }
    },
    trader3: {
      exec: 'buy_order',
      executor: 'constant-vus',
      vus: 1,
      duration: '3s',
      gracefulStop: '2s',
      env: {
        URL: 'ws://127.16.123.3:4000/orders/ws',
        TRADERID: 'Columbia_C',
      }
    },
    trader4: {
      exec: 'buy_order',
      executor: 'constant-vus',
      vus: 1,
      duration: '3s',
      gracefulStop: '2s',
      env: {
        URL: 'ws://127.16.123.4:4000/orders/ws',
        TRADERID: 'Columbia_D',
      }
    }
  },
};

function getRandomInt(max) {
  return Math.floor(Math.random() * max);
}

export function buy_order() {
  // url = __ENV.URL;
  // url = 'ws://127.16.123.1:4000/orders/ws';
  const res = ws.connect(__ENV.URL, null, function (socket) {
    socket.on('open', function open() {
      console.log('connected')
      socket.setInterval(function interval() {
        
        socket.send(JSON.stringify({
          'OrderType': Math.random() < 0.5 ? "Buy" : "Sell",
          'Amount': 1 + getRandomInt(9),
          'Price': 1 + getRandomInt(9),
          'Symbol': "AAPL",
          'TraderId': __ENV.TRADERID,
        }));
      }, .01);
    });
  });

  check(res, { 'status is 101': (r) => r && r.status === 101 });
}