#![allow(dead_code)]
#[warn(unused_attributes)]
extern crate hyper;
extern crate jsonrpc2;

use jsonrpc2::{ 
    JsonRpc, RpcResult, Request, Response, 
    Error as RpcError, Json, ToJson 
};

use std::io::{ copy, Read, Write };
use std::sync::{ Arc }; // Mutex
use std::env;
use std::str::FromStr;

use hyper::server::{ 
    Server, Request as HyperRequest, 
    Response as HyperResponse, 
    Handler as HyperHandler
};
use hyper::method::Method::{ Get, Put, Post };
use hyper::status::StatusCode; // { Ok, BadRequest, NotFound, MethodNotAllowed };


// TODO: 给 RPC Method 增加 Share Memeory 支持.
//          fn hello (sm: ShareMemoy , params: Json) -> RpcResult { }
fn hello (params: &Option<Json>) -> RpcResult {
    match params.as_ref() {
        Some(p) => {
            println!("Params: {:?}", p );
        },
        None => {
            println!("Params: Null" );
        }
    };
    Ok("Hello World".to_json())
}

fn ice (params: &Option<Json>) -> RpcResult {
    match params.as_ref() {
        Some(p) => {
            println!("Params: {:?}", p );
        },
        None => {
            println!("Params: Null" );
        }
    };
    let data = vec![1,2,3,4];
    Ok( data.to_json() )
}

struct MyHandler {
    rpc: Arc<JsonRpc>
}
impl HyperHandler for MyHandler {
    fn handle(&self, mut req: HyperRequest, mut res: HyperResponse) {
        println!("{:?}", self.rpc.methods().len() );
        match req.method {
            Post | Put => {
                println!("{:?}", req.headers);

                let mut body = String::new();
                match req.read_to_string(&mut body){
                    Ok(body_length) => {
                        println!("Body length: {:?}\n{:?}", body_length, body);
                        let mut res = &mut res.start().unwrap();
                        match Request::from_str(&body) {
                            Ok(rpc_request) => {
                                let response_content = self.rpc.call(&rpc_request).to_json().to_string();
                                res.write(response_content.as_bytes()).unwrap();
                            },
                            Err(rpc_error) => {
                                println!("Error:   {:?}", rpc_error);
                                res.write(rpc_error.to_json().to_string().as_bytes()).unwrap();
                                // *res.status_mut() = StatusCode::MethodNotAllowed;
                            }
                        }
                        // res.status_mut() = &mut StatusCode::Ok;
                    },
                    Err(e) => {
                        println!("Bad Request. {:?}", e);
                        let mut res = &mut res.start().unwrap();
                        // res.status_mut() = StatusCode::BadRequest;
                        res.write(e.to_string().as_bytes()).unwrap();
                    }
                }
            },
            Get => {
                println!("{:?}", req.headers);
                copy(&mut req, &mut res.start().unwrap()).unwrap();  
            },
            _ => {
                println!("Method Not Allowed.");
                *res.status_mut() = StatusCode::MethodNotAllowed;
            }
        };
    }
}
unsafe impl Send for MyHandler { }
unsafe impl Sync for MyHandler { }


fn main (){
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);

    let mut rpc = JsonRpc::new();
    rpc.register("hello", Box::new(hello));
    rpc.register("ice", Box::new(ice));

    let share_rpc = Arc::new(rpc);
    Server::http("0.0.0.0:80").unwrap().handle( MyHandler{ rpc: share_rpc.clone() } ).unwrap();    
    // Server::http("0.0.0.0:80").unwrap().handle( move |req: Request, res: Response| {
    //     println!("{:?}", share_rpc.methods().len() );
    //     *res.status_mut() = StatusCode::MethodNotAllowed;
    // }).unwrap();
}