#![feature(box_syntax)]
#![allow(dead_code)]
#[warn(unused_attributes)]


mod jsonrpc;
use jsonrpc::{ JsonRpc, RpcResult, RpcRequest, 
    json, ToJson };

// TODO: 给 RPC Method 增加 Share Memeory 支持.
//          fn hello (sm: ShareMemoy , params: json::Json) -> RpcResult { }
fn hello (params: &Option<json::Json>) -> RpcResult {
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

fn ice (params: &Option<json::Json>) -> RpcResult {
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

fn main (){
    let mut rpc = JsonRpc::new();
    rpc.add_method("hello", Box::new(hello));
    rpc.add_method("ice", Box::new(ice));
    let request1 = "{\"params\": [\"参数1\", \"param 2\"], \"jsonrpc\": \"2.0\", \"method\": \"ice\", \"id\": 1}";
    match RpcRequest::new(&request1) {
        Ok(rpc_request) => {
            println!("{}", rpc.call(&rpc_request).to_json().to_string() );
        },
        Err(e) => {
            println!("Error:   {:?}", e);
        }
    }
}