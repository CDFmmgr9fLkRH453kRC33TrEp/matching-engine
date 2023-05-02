use fefix::prelude::*;
use fefix::tagvalue::Decoder;
use fefix::tagvalue::Config;
use fefix::tagvalue::MessageGroup;

// use decimal::d128;
use fefix::prelude::*;
use fefix::tagvalue::Encoder;

use crate::OrderRequest;
use crate::orderbook::Order;
// use rust_decimal_macros::dec;

// 8=FIX.4.4|9=122|35=D|34=215|49=CLIENT12|52=20100225-19:41:57.316|56=B|1=Marcel|11=13346|21=1|40=2|44=5|54=1|59=0|60=20100225-19:39:52.020|10=072|

fn encode() -> Vec<u8>
{
    let mut encoder = Encoder::<Config>::default();
    let mut buffer = Vec::new();
    let mut msg = encoder.start_message(b"FIX.4.2", &mut buffer, b"D");
    
    msg.set(fix42::MSG_SEQ_NUM, 215);
    msg.set(fix42::TARGET_COMP_ID, "B");
    
    msg.set(fix42::SENDER_COMP_ID, "CompID Test");
    msg.set(fix42::ORDER_ID, "OrderID Test");
    msg.set(fix42::SYMBOL, "Symbol Test");
    msg.set(fix42::ORDER_QTY, 12);
    msg.set(fix42::PRICE, 12);
    msg.set(fix42::ORD_TYPE, 3);

    // println!("Message Type: {:?}", msg.fv::<fix42::MsgType>(fix42::MSG_TYPE).unwrap());
    // println!("Sender ID: {:?}", msg.fv::<&str>(fix42::SENDER_COMP_ID).unwrap());
    // // println!("Order ID: {:?}", msg.fv::<&str>(fix42::ORDER_ID).unwrap());
    // println!("Asset ID: {:?}", msg.fv::<&str>(fix42::SYMBOL).unwrap());
    // println!("Quantity: {:?}", msg.fv::<u64>(fix42::ORDER_QTY).unwrap());
    // println!("Price: {:?}", msg.fv::<u64>(fix42::PRICE).unwrap());
    // println!("OrderType: {:?}", msg.fv::<fix42::OrdType>(fix42::ORD_TYPE).unwrap());
    // println!("Wrapped: {:?}", msg.wrap());
    msg.wrap().to_owned()
}



// const FIX_MESSAGE: &[u8] = b"8=FIX.4.2|9=196|35=X|49=A|56=B|34=12|52=20100318-03:21:11.364|262=A|268=2|279=0|269=0|278=BID|55=EUR/USD|270=1.37215|15=EUR|271=2500000|346=1|279=0|269=1|278=OFFER|55=EUR/USD|270=1.37224|15=EUR|271=2503200|346=1|10=171|";
// const FIX_MESSAGE: &[u8] = b"8=FIX.4.29=10835=D49=A56=B34=1238=10052=20100318-03:21:11.36411=12321=255=AAPL54=160=20100318-03:21:11.36440=710=065";

// 8=begin str
// 9=body len
// 35=msgtype
// 49=sender comp id
// 56=target id (should always be our exchange for asset)
// 34=message sequence number?
// 52=sending time
// 262=market data request id (i.e. which asset)
// 268=number of market data entries
// 10=checksum (replace | with \001 (SOH) when calculating)


// we only need to support the following incoming message types:
// D=new single order request
    // 11=client generated order id (created by joining trader_id and incremented order)
    // 55=symbol
    // 54=side
    // 60=transaction time
    // 38=order quantity
    // 40=order type
    // 44=price
// F=order cancel request
    // 41=target order's <11> id
    // 11=this cancel request's order id
    // 55=symbol
    // 54=side
    // 44=price
    // 60=transaction
    // 38=order quantity 
    // 21=handling instructions (should be 2 = Automated execution order, public)
// G=order cancel/replace request
    // 41=target order's <11> id
    // 11=this cancel request's order id
    // 55=symbol
    // 54=side
    // 44=price
    // 60=transaction
    // 38=order quantity    
// V=market data request (this is iffy, I think we can assume everyone is subscribed to everything)

// and the following outgoing message types:
// 9=order cancel reject
// 8=execution report (could be rejection)
// 3=message reject (i.e. for malformed request)
// X=incremental refresh market data
// W=full refresh market data


fn main() {
    let fix_dictionary = Dictionary::fix42();
    // Let's create a FIX decoder. This is an expensive operation, and it should
    // only be done once at the beginning of your program and/or FIX session.
    let mut fix_decoder = Decoder::<Config>::new(fix_dictionary);
    // In this case, the FIX message is specified using "|" rather than SOH
    // (ASCII 0x1) bytes. FerrumFIX supports this.
    let FIX_MESSAGE: Vec<u8> = encode();
    fix_decoder.config_mut().set_separator(b'');
    let msg = fix_decoder
        .decode(FIX_MESSAGE.as_slice())
        .expect("Invalid FIX message");

    // Read the FIX message! You get nice type inference out of the box. You
    // have fine-grained control over how to decode each field, even down to raw
    // bytes if you want full control.
    assert_eq!(msg.fv(fix42::BEGIN_STRING), Ok(b"FIX.4.2"));
    assert_eq!(msg.fv(fix42::MSG_TYPE), Ok(b"D"));
    assert_eq!(
        msg.fv(fix42::MSG_TYPE),
        Ok(fix42::MsgType::OrderSingle)
    );
    assert_eq!(msg.fv(fix42::SENDER_COMP_ID), Ok(b"CompID Test"));
    assert_eq!(msg.fv(fix42::TARGET_COMP_ID), Ok(b"B"));
    assert_eq!(msg.fv(fix42::MSG_SEQ_NUM), Ok(215));

    println!("Message Type: {:?}", msg.fv::<fix42::MsgType>(fix42::MSG_TYPE).unwrap());
    println!("Sender ID: {:?}", msg.fv::<&str>(fix42::SENDER_COMP_ID).unwrap());
    // println!("Order ID: {:?}", msg.fv::<&str>(fix42::ORDER_ID).unwrap());
    println!("Asset ID: {:?}", msg.fv::<&str>(fix42::SYMBOL).unwrap());
    println!("Quantity: {:?}", msg.fv::<u64>(fix42::ORDER_QTY).unwrap());
    println!("Price: {:?}", msg.fv::<u64>(fix42::PRICE).unwrap());
    println!("OrderType: {:?}", msg.fv::<fix42::OrdType>(fix42::ORD_TYPE).unwrap());
    
    
}


pub fn parse_bytes(bytes: &[u8]) -> Option<OrderRequest>{
    let fix_dictionary = Dictionary::fix42();
    // Let's create a FIX decoder. This is an expensive operation, and it should
    // only be done once at the beginning of your program and/or FIX session.
    let mut fix_decoder = Decoder::<Config>::new(fix_dictionary);
    // In this case, the FIX message is specified using "|" rather than SOH
    // (ASCII 0x1) bytes. FerrumFIX supports this.
    let FIX_MESSAGE: Vec<u8> = encode();
    fix_decoder.config_mut().set_separator(b'');
    let msg = fix_decoder
        .decode(FIX_MESSAGE.as_slice())
        .expect("Invalid FIX message");

    let order_request = OrderRequest{
        symbol:     msg.fv::<&str>(fix42::SYMBOL).unwrap(),
        amount:     msg.fv::<u64>(fix42::ORDER_QTY).unwrap(),
        price:      msg.fv::<u64>(fix42::PRICE).unwrap(),
        order_type: msg.fv::<fix42::OrdType>(fix42::ORD_TYPE).unwrap(),
        trader_id:  msg.fv::<&str>(fix43::SENDER_COMP_ID).unwrap()
    };

    Some(order_request)

}


#[cfg(test)]
#[test]
fn run() {
    main();
}