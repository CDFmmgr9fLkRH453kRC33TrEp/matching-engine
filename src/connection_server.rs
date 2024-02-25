/// main actor which manages connections and order flow data messages
/// transaction updates are sent from the orderbook add_order fn to this actor
/// this actor then fairly sends out transaction updates to all connected websockets.
// use actix_web::*;
use actix::*;
use std::sync::Arc;

use crate::message_types::OpenMessage;

pub struct Server {
    connected_actors: Vec<Recipient<Arc<crate::orderbook::LimLevUpdate>>>,
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