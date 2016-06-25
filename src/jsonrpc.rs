#![allow(dead_code)]

extern crate rustc_serialize;

use std::collections::BTreeMap;
pub use self::rustc_serialize::{ json };
// use std::sync::Arc;
pub use self::rustc_serialize::json:: { ToJson };

#[derive(Debug)]
pub struct RpcError {
    code : i64,
    message : String,
    data : Option<json::Json>,
}
impl RpcError {
    pub fn to_json(&self) -> json::Json {
        let mut d = BTreeMap::new();
        d.insert("code".to_string(),    json::Json::I64(self.code));
        d.insert("message".to_string(), json::Json::String(self.message.clone()));
        match self.data {
            Some(ref data) => {
                d.insert("data".to_string(),  data.clone() );
            },
            None => {
                d.insert("data".to_string(),  json::Json::Null );
            }
        }
        json::Json::Object(d)
    }
}

// pub enum RpcResult {
//     json::Json,

// }
pub type RpcResult = Result<json::Json, &'static str>;
pub type RpcHandle = Box<Fn(&Option<json::Json>)-> RpcResult>;

#[derive(Debug)]
pub struct SuccessResponse {
    jsonrpc: String,
    result : Option<json::Json>,
    id     : Option<i64>
}
impl SuccessResponse {
    pub fn to_json(&self) -> json::Json {
        let mut d = BTreeMap::new();
        d.insert("jsonrpc".to_string(), json::Json::String(self.jsonrpc.clone()) );
        match self.result {
            Some(ref result) => {
                d.insert("result".to_string(),  result.clone() );
            },
            None => {
                d.insert("result".to_string(),  json::Json::Null );
            }
        }
        match self.id {
            Some(ref id) => {
                d.insert("id".to_string(),  json::Json::I64(id.clone()) );
            },
            None => {
                d.insert("id".to_string(),  json::Json::Null );
            }
        }
        json::Json::Object(d)
    }
}

#[derive(Debug)]
pub struct ErrorResponse {
    jsonrpc: String,
    error  : RpcError,
    id     : Option<i64>
}
impl ErrorResponse {
    pub fn to_json(&self) -> json::Json {
        let mut d = BTreeMap::new();
        d.insert("jsonrpc".to_string(), json::Json::String(self.jsonrpc.clone()) );
        d.insert("error".to_string(),   self.error.to_json() );
        match self.id {
            Some(ref id) => {
                d.insert("id".to_string(),  json::Json::I64(id.clone()) );
            },
            None => {
                d.insert("id".to_string(),  json::Json::Null );
            }
        }
        json::Json::Object(d)
    }
}

#[derive(Debug)]
pub enum RpcErrorCode {
    ParseError,
    InvalidRequest,
    MethodNotFound,
    InvalidParams,
    InternalError,
    ServerError
}

#[derive(Debug)]
pub struct RpcRequest {
    jsonrpc: String,
    method : String,
    params : Option<json::Json>,
    id     : Option<i64>
}

impl RpcRequest {
    pub fn new (jstring: &str) -> Result<RpcRequest, RpcError> {
        match json::Json::from_str(jstring) {
            Ok(j) => {
                match RpcRequest::parse(j) {
                    Ok(rpc_request) => {
                        Ok(rpc_request)
                    },
                    Err(rpc_error_code) => {
                        // // -32000 to -32099    Server error        Reserved for implementation-defined server-errors.
                        Err(match rpc_error_code {
                            RpcErrorCode::ParseError => {
                                RpcError { code: -32700, message: "Parse error".to_string(), data: None }
                            },
                            RpcErrorCode::InvalidRequest => {
                                RpcError { code: -32600, message: "Invalid Request".to_string(), data: None }
                            },
                            RpcErrorCode::MethodNotFound => {
                                RpcError { code: -32601, message: "Method not found".to_string(), data: None }
                            },
                            RpcErrorCode::InvalidParams => {
                                RpcError { code: -32602, message: "Invalid method parameter(s)".to_string(), data: None }
                            },
                            RpcErrorCode::InternalError => {
                                RpcError { code: -32603, message: "Internal error".to_string(), data: None }
                            },
                            RpcErrorCode::ServerError => {
                                RpcError { code: -32000, message: "Server error".to_string(), data: None }
                            }
                        })
                    }
                }
            },
            Err(_) => {
                // Err(RpcErrorCode::ParseError)
                Err(RpcError { code: -32700, message: "Parse error".to_string(), data: None })
            }
        }
    }
    pub fn parse(j: json::Json) -> Result<RpcRequest, RpcErrorCode> {
        /*
        http://www.jsonrpc.org/specification#request_object
            -32700              Parse error         Invalid JSON was received by the server.
                                                    An error occurred on the server while parsing the JSON text.
            -32600              Invalid Request     The JSON sent is not a valid Request object.
            -32601              Method not found    The method does not exist / is not available.
            -32602              Invalid params      Invalid method parameter(s).
            -32603              Internal error      Internal JSON-RPC error.
            -32000 to -32099    Server error        Reserved for implementation-defined server-errors.
        */
        let obj = match j.as_object() {
            Some(obj) => obj,
            None => return Err(RpcErrorCode::ParseError)
        };
        let version = RpcRequest::_parse_version(obj);
        let method  = RpcRequest::_parse_method(obj);
        let params  = RpcRequest::_parse_params(obj);
        let id      = RpcRequest::_parse_id(obj);
        if version.is_ok() && method.is_ok() && params.is_ok() && id.is_ok() {
            Ok(RpcRequest {
                jsonrpc: version.ok().unwrap(),
                method : method.ok().unwrap(),
                params : params.ok().unwrap(),
                id     : id.ok().unwrap()
            })
        } else {
            Err(RpcErrorCode::InvalidRequest)
        }
    }
    fn _parse_version (obj: &json::Object) -> Result<String, RpcErrorCode> {
        match obj.get("jsonrpc") {
            Some(version) => {
                if json::Json::String("2.0".to_string()) == *version 
                || json::Json::String("2".to_string()) == *version  {
                    Ok("2.0".to_string())
                } else {
                    // JsonRpc Version Must Be 2.0. 
                    Err(RpcErrorCode::InvalidRequest)
                }
            },
            None => {
                Err(RpcErrorCode::InvalidRequest)
            }
        }
    }
    fn _parse_method (obj: &json::Object) -> Result<String, RpcErrorCode> {
        match obj.get("method") {
            Some(m) => {
                  if m.is_string() {
                    Ok(m.as_string().unwrap().to_string())
                  } else {
                    Err(RpcErrorCode::InvalidRequest)
                  }
            },
            None => {
                Err(RpcErrorCode::InvalidRequest)
            }
        }
    }
    fn _parse_params (obj: &json::Object) -> Result<Option<json::Json>, RpcErrorCode> {
        match obj.get("params") {
            Some(p) => {
                if p.is_array() || p.is_object() {
                    Ok(Some(p.clone()))
                } else if p.is_null() {
                    Ok(None)
                } else {
                    Err(RpcErrorCode::InvalidParams)
                }
            },
            None => {
                Err(RpcErrorCode::InvalidRequest)
            }
        }
    }
    fn _parse_id (obj: &json::Object) -> Result<Option<i64>, RpcErrorCode> {
        match obj.get("id") {
            Some(i) => {
                // i.is_number() || i.is_u64()
                if i.is_i64() || i.is_u64() {
                    Ok(Some(i.as_i64().unwrap()))
                } else if i.is_null() {
                    Ok(None)
                } else {
                    Err(RpcErrorCode::InvalidRequest)
                }
            },
            None => {
                Err(RpcErrorCode::InvalidRequest)
            }
        }
    }
}

#[derive(Debug)]
pub enum RpcResponse {
    SuccessResponse(SuccessResponse),
    ErrorResponse(ErrorResponse)
}
impl  RpcResponse {
    pub fn to_json(&self) -> json::Json {
        match self {
            &RpcResponse::SuccessResponse(ref r) => {
                r.to_json()
            },
            &RpcResponse::ErrorResponse(ref r) => {
                r.to_json()
            }
        }
    }
}


pub struct JsonRpc {
    methods : BTreeMap<String, RpcHandle>
}
impl JsonRpc {
    pub fn new () -> JsonRpc {
        JsonRpc {  methods : BTreeMap::new() }
    }
    pub fn register (&mut self, method: &str, handle: RpcHandle) {
        self.methods.insert(method.to_string(), handle);
    }
    pub fn methods(&self) -> &BTreeMap<String, RpcHandle> {
        &self.methods
    }
    pub fn call(&self, request: &RpcRequest) -> RpcResponse {
        match self.methods.get(&request.method) {
            Some(func) => {
                match func(&request.params) {
                    Ok(rpc_result) => {
                        RpcResponse::SuccessResponse (SuccessResponse {
                            jsonrpc: request.jsonrpc.clone(),
                            result: Some(rpc_result),
                            id: request.id.clone()
                        })
                    },
                    Err(e) => {
                        RpcResponse::ErrorResponse(ErrorResponse {
                            jsonrpc: request.jsonrpc.clone(),
                            error: RpcError {code: -32603, message: e.to_string(), data: None},
                            id: request.id.clone()
                        })
                    }
                }
            },
            None => {
                RpcResponse::ErrorResponse( ErrorResponse {
                    jsonrpc: request.jsonrpc.clone(),
                    error: RpcError { code: -32601, message: "Method not found".to_string(), data: None },
                    id: request.id.clone()
                })
            }
        }

    }
}
unsafe impl Send for JsonRpc { }
unsafe impl Sync for JsonRpc { }




// fn main (){
//     let mut rpc = JsonRpc::new();
//     rpc.register("hello", Box::new(hello));
//     rpc.register("ice", Box::new(ice));
//     let request1 = "{\"params\": [\"参数1\", \"param 2\"], \"jsonrpc\": \"2.0\", \"method\": \"ice\", \"id\": 1}";
//     match RpcRequest::new(&request1) {
//         Ok(rpc_request) => {
//             println!("{}", rpc.call(&rpc_request).to_json().to_string() );
//         },
//         Err(e) => {
//             println!("Error:   {:?}", e);
//         }
//     }
// }