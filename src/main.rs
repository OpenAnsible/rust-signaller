
// #[macro_use(debug, error, info, log, log_enabled, trace, warn)]
// extern crate log;
// extern crate env_logger;
extern crate ws;
extern crate bson;
extern crate rustc_serialize;

use std::str::FromStr;
use std::string::ToString;

pub use std::net::SocketAddr;

pub use bson::oid::ObjectId;
pub use rustc_serialize::{json, Decodable, Encodable};
pub use rustc_serialize::json::{Json, ToJson, Object};
pub use rustc_serialize::hex::ToHex;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
pub use std::collections::BTreeMap;

mod message;

pub type Registry = Rc<RefCell<HashMap<ObjectId, ws::Sender>>>;

struct Server {
    oid: ObjectId,
    out: ws::Sender,
    registry: Registry
}

impl ws::Handler for Server {
    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        println!("Peer Addr: {:?} Token: {:?} oid: {:?}", 
            shake.remote_addr().unwrap().unwrap().to_string(), self.out.token(), self.oid );
        // println!("Peer Socket Addr: {:?}", shake.local_addr);
        // self.out.close(ws::CloseCode::Normal);
        self.registry.borrow_mut().insert(self.oid.clone(), self.out.clone());
        Ok(())
    }
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        match msg.as_text() {
            Ok(msg) => {
                match message::Request::from_str(&msg){
                    Ok(req) => {
                        match message::Response::from_request(&req, &self.registry.clone(), self.oid.clone() ){
                            Ok(res) => {
                                self.out.send( res.to_json().to_string() )
                            },
                            Err(_)  => {
                                self.out.send(format!("Response Error.") )
                            }
                        }
                    },
                    Err(_) => {
                        self.out.send(format!("Request Parse Error.") )
                    }
                }
            },
            Err(_) => {
                self.out.send(format!("ASCII Error.") )
            }
        }
    }
    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        println!("WebSocket closing by ({:?}) for ({:?}:{}) ", self.out.token(), code, reason);
        self.registry.borrow_mut().remove(&self.oid);
    }
    fn on_error(&mut self, err: ws::Error) {
        println!("{:?}", err);
    }
}

impl Server {
    fn run(host: &str){
        let registry: Registry = Rc::new(RefCell::new(HashMap::new()));
        println!("WebSocket Server running on {}", host);
        let ws = ws::WebSocket::new(|out|{
            Server { 
                oid: bson::oid::ObjectId::new().unwrap(), 
                out: out, 
                registry: registry.clone()
            }
        });
        match ws {
            Ok(ws) => {
                let _ = ws.listen(host);
                println!("WebSocket Server running on {}", host);
            },
            Err(e) => println!("WebSocket Server Running Error ({:?})", e)
        };
    }
}

fn main (){
    // let args: Vec<String> = env::args().collect();
    // env_logger::init().unwrap();

    let host = "127.0.0.1:3012";
    Server::run(host);
}