import ws from 'k6/ws';
import { check } from 'k6';

// maxes out around 10.7k/s

export let options = {
  stages: [
    { duration: '3s', target: 1 },
    { duration: '3s', target: 0 },
  ],
};

export default function () {
  const text = JSON.stringify({    
        'OrderType': "Buy",
        'Amount': 1,
        'Price': 1,
        'Symbol': "AAPL",
        'TraderId': "Columbia_A",
  })
  // public websocket server for quick test
  //const url = 'wss://javascript.info/article/websocket/demo/hello';
  const url = 'ws://127.16.123.1:4000/orders/ws';    // local websocket server

  const res = ws.connect(url, null, function (socket) {
    socket.on('open', function open() {
      console.log('connected');
      socket.setInterval(function interval() {
        socket.send(text);
        // console.log('Order sent: ', text);
      }, .0001);
    });

    socket.on('message', function message(data) {
    //   console.log('Message received: ', data);
    //   check(data, { 'data is correct': (r) => r && r === text });
    });

    socket.on('close', () => console.log('disconnected'));

    socket.setTimeout(function () {
      console.log('5 seconds passed, closing the socket');
      socket.close();
    }, 5000);
  });

  check(res, { 'status is 101': (r) => r && r.status === 101 });
}