
#[macro_use(debug, error, info, log, log_enabled, trace, warn)]
extern crate log;
extern crate env_logger;
extern crate ws;
extern crate bson;

use std::str::FromStr;
use std::string::ToString;
pub use std::collections::BTreeMap;
pub use std::net::SocketAddr;

use ws::{ // connect, listen
    WebSocket, CloseCode, 
    Handler, Handshake,
    Message, Result, Error
};

pub use ws::Sender;
pub use ws::util::Token;
pub use bson::oid::ObjectId;
pub use rustc_serialize::json::{Json, ToJson, Object};
pub use rustc_serialize::hex::ToHex;

mod channels;
mod peer;

pub use channels::{Channel, Channels};
pub use peer::{Peer, Peers};


struct Server {
    out: Sender,
    channels: Channels
}

impl Handler for Server {
    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        println!("Peer Addr: {:?} Token: {:?}", 
            shake.remote_addr().unwrap().unwrap().to_string(), self.out.token() );
        // println!("Peer Socket Addr: {:?}", shake.local_addr);

        Ok(())
    }
    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Server got message '{}'. ", msg);
        self.out.send(msg)
    }
    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("WebSocket closing by ({:?}) for ({:?}:{}) ", self.out.token(), code, reason);
        // self.out.shutdown().unwrap();
    }
    fn on_error(&mut self, err: Error) {
        println!("{:?}", err);
    }
}

impl Server {
    fn run(host: &str){
        // listen(host, |out| { Server { out: out } })
        let ws = WebSocket::new(|out|{
            Server { out: out, channels: Channels::empty() }
        });
        match ws {
            Ok(ws) => {
                let _ = ws.listen(host);
                println!("WebSocket Server running on {}", host);
            },
            Err(e) => println!("WebSocket Server Running Error ({:?})", e)
        };
    }
    fn channels(&self) -> Channels {
        self.channels
    }
    fn join(&self, channel_id: ObjectId, token: Option<String>, ) -> Result<Vec<Channels>> {
        
    }
    fn leave(&self, channel_id: ObjectId) -> bool {
        false
    }
    fn broadcast(&self, channel_id: ObjectId) -> bool {
        false
    }
}

fn main (){
    // let args: Vec<String> = env::args().collect();
    env_logger::init().unwrap();

    let host = "127.0.0.1:3012";
    Server::run(host);
}