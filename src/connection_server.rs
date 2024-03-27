/// main actor which manages connections and order flow data messages
/// transaction updates are sent from the orderbook add_order fn to this actor
/// this actor then fairly sends out transaction updates to all connected websockets.
// use actix_web::*;
use actix::*;
use std::sync::Arc;

use crate::{api_messages::OutgoingMessage, message_types::OpenMessage, orderbook::LimLevUpdate};

pub struct Server {
    connected_actors: Vec<Recipient<Arc<LimLevUpdate>>>,
}

impl Server {
    pub fn new() -> Server {
        warn!("Relay Server actor created");
        Server {
            // todo: capacity number should be abstracted to config file. 
            
            connected_actors: Vec::with_capacity(1000)
        }
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}

// need to impl all "OutgoingOrderMessage"
impl Handler<crate::orderbook::LimLevUpdate> for Server {
    // forward limit level updates to all connected actors
    type Result = ();

    fn handle(
        &mut self,
        msg: crate::orderbook::LimLevUpdate,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        debug!("New LimLevUpdate Message Received by Relay Server");
        let msg_arc = Arc::new(msg);
        for connection in self.connected_actors.iter() {
            connection.do_send(msg_arc.clone());
        }
    }
}


impl Handler<Arc<OutgoingMessage<'_>>> for Server {
    type Result = ();
    fn handle(&mut self, msg: Arc<OutgoingMessage>, ctx: &mut Self::Context) {      
        // there has to be a nicer way to do this, but cant figure out how to access inner type when doing a default match
        // ctx.text(serde_json::to_string(&msg.d).unwrap());

        match *msg {
            OutgoingMessage::NewRestingOrderMessage(m) => {
                let msg_arc = Arc::new(m);
                for connection in self.connected_actors.iter() {
                    connection.do_send(msg_arc.clone());
                }
            }
            OutgoingMessage::TradeOccurredMessage(m) =>  {
                ctx.text(serde_json::to_string(&m).unwrap());
            }
            OutgoingMessage::CancelOccurredMessage(m) => {
                ctx.text(serde_json::to_string(&m).unwrap());
            },
            OutgoingMessage::OrderFillMessage(m) => {
                ctx.text(serde_json::to_string(&m).unwrap());
            },
            OutgoingMessage::OrderConfirmMessage(m) => {
                ctx.text(serde_json::to_string(&m).unwrap());
            },
            OutgoingMessage::OrderPlaceErrorMessage(m) => {
                ctx.text(serde_json::to_string(&m).unwrap());
            },
            OutgoingMessage::CancelConfirmMessage(m) => {
                ctx.text(serde_json::to_string(&m).unwrap());
            },
            OutgoingMessage::CancelErrorMessage(m) => {
                ctx.text(serde_json::to_string(&m).unwrap());
            },
            
        }
    }
}

// TODO: handle websocket disconnects by removing actors from list
impl Handler<crate::message_types::OpenMessage> for Server{
    type Result = ();
    fn handle(
        &mut self,
        msg: OpenMessage,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        let res = self.connected_actors.push(msg.addr);
        debug!("New websocket actor registered!");
        debug!("Full list: {:?}", &self.connected_actors);        
        res
    }
}