
use std::str::FromStr;
use std::string::ToString;

use ::{ ObjectId, Json, ToJson, ToHex, Registry, BTreeMap };
use ::ws::Sender;
use ::rustc_serialize::json;

#[derive(Clone, Debug)]
pub enum Event {
    Cmd,
    Msg,
    Sdp,
    Candidate
}

impl ToJson for Event {
    fn to_json(&self) -> Json {
        match *self {
            Event::Cmd       => Json::String("cmd".to_string()),
            Event::Msg       => Json::String("msg".to_string()),
            Event::Sdp       => Json::String("sdp".to_string()),
            Event::Candidate => Json::String("candidate".to_string())
        }
    }
}

impl FromStr for Event {
    type Err = ();
    fn from_str(s: &str) -> Result<Event, ()> {
        match s {
            "cmd" => Ok(Event::Cmd),
            "msg" => Ok(Event::Msg),
            "sdp" => Ok(Event::Sdp),
            "candidate" => Ok(Event::Candidate),
            _     => Err(())
        }
    }
}

#[derive(Clone, Debug)]
pub struct Request {
    id     : String,
    event  : Event,
    target : Option<ObjectId>,
    content: Option<String>
}

impl ToJson for Request {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("id".to_string(),      self.id.to_json() );
        d.insert("event".to_string(),   self.event.to_json() );
        match self.target {
            Some(ref target) => {
                d.insert("target".to_string(),  target.to_hex().to_json() );
            },
            None => {}
        };
        d.insert("content".to_string(), self.content.to_json() );
        Json::Object(d)
    }
}
impl FromStr for Request {
    type Err = ();
    fn from_str(s: &str) -> Result<Request, ()> {
        let data = match Json::from_str(s) {
            Ok(data) => data,
            Err(_)   => return Err(())
        };
        if !data.is_object() {
            return Err(());
        }
        let obj = match data.as_object() {
            Some(obj) => obj,
            None      => return Err(())
        };
        let id = match obj.get("id") {
            Some(id) => match id.as_string(){
                Some(id)=> id.to_string(),
                None    => return Err(())
            },
            None     => return Err(())
        };
        let event = match obj.get("event") {
            Some(event) => match event.as_string(){
                Some(ref event) => match Event::from_str(event){
                    Ok(evt) => evt,
                    Err(_)  => return Err(())
                },
                None => return Err(())
            },
            None        => return Err(())
        };
        let target = match obj.get("target") {
            Some(target) => match target.as_string(){
                Some(target) => match ObjectId::with_string(target){
                    Ok(oid)  => Some(oid),
                    Err(_)   => return Err(())
                },
                None => return Err(())
            },
            None         => None
        };
        let content = match obj.get("content") {
            Some(content) => match content.as_string(){
                Some(ref content) => Some(content.to_string()),
                None => None
            },
            None          => None
        };
        Ok(Request {
            id     : id,
            event  : event,
            target : target,
            content: content
        })
    }
}

impl Request {
    pub fn id(&self) -> String {
        self.id.clone()
    }
    pub fn event(&self) -> Event {
        self.event.clone()
    }
    pub fn target(&self) -> Option<ObjectId> {
        self.target.clone()
    }
    pub fn content(&self) -> Option<String> {
        self.content.clone()
    }
}


#[derive(Clone, Debug)]
pub struct ErrorResult {
    code   : usize,
    message: Option<String>
}

#[derive(Clone, Debug)]
pub enum Response {
    Success(Option<String>, Json),
    Error(Option<String>, Json)
}

impl ToJson for Response {
    fn to_json(&self) -> Json {
        match *self {
            Response::Success(ref id, ref res) => {
                let mut r = BTreeMap::new();
                r.insert("id".to_string(),     id.to_json() );
                r.insert("result".to_string(), res.clone() );
                Json::Object(r)
            },
            Response::Error(ref id, ref error) => {
                let mut r = BTreeMap::new();
                r.insert("id".to_string(),    id.to_json() );
                r.insert("error".to_string(), error.clone() );
                Json::Object(r)
            }
        }
    }
}
impl Response {
    pub fn from_request(req: &Request, 
                        registry: &Registry, 
                        from: ObjectId) -> Result<Response, ()> {
        match req.event() {
            Event::Cmd => {
                match req.content() {
                    Some(ref content) => match content.as_ref() {
                        "peers" => {
                            let mut peers = registry.borrow().keys().cloned().collect::<Vec<ObjectId>>()
                                .iter().map(|&ref oid| oid.to_hex().to_json() ).collect::<Vec<Json>>();
                            peers.retain(|&ref oid| oid.to_string() != from.to_hex().to_json().to_string() );
                            Ok(Response::Success(Some(req.id()), Json::Array(peers) ))
                        },
                        _       => {
                            let mut e = BTreeMap::new();
                            e.insert("code".to_string(),    404usize.to_json() );
                            e.insert("message".to_string(), "指令不存在".to_json() );
                            Ok(Response::Error(Some(req.id()), Json::Object(e)))
                        }
                    },
                    None => {
                        let mut e = BTreeMap::new();
                        e.insert("code".to_string(),    404usize.to_json() );
                        e.insert("message".to_string(), "指令不存在".to_json() );
                        Ok(Response::Error(Some(req.id()), Json::Object(e)))
                    }
                }
            },
            Event::Sdp => {
                match req.target(){
                    Some(ref target) => match registry.borrow().get(target){
                        Some(target_conn) => {
                            let mut obj = req.to_json().as_object_mut().unwrap().clone();
                            obj.insert("from".to_string(), from.to_hex().to_json() );

                            target_conn.send(Json::Object(obj.clone()).to_string());
                            Ok(Response::Success(Some(req.id()), Json::String("ok".to_string())))
                        },
                        None              => {
                            let mut e = BTreeMap::new();
                            e.insert("code".to_string(),    404usize.to_json() );
                            e.insert("message".to_string(), "目标不存在".to_json() );
                            Ok(Response::Error(Some(req.id()), Json::Object(e)))
                        }
                    },
                    None => {
                        let mut e = BTreeMap::new();
                        e.insert("code".to_string(),    400usize.to_json() );
                        e.insert("message".to_string(), "未指定目标".to_json() );
                        Ok(Response::Error(Some(req.id()), Json::Object(e)))
                    }
                }
            },
            Event::Msg => {
                let mut e = BTreeMap::new();
                e.insert("code".to_string(),    500usize.to_json() );
                e.insert("message".to_string(), "功能未实现".to_json() );
                Ok(Response::Error(Some(req.id()), Json::Object(e)))
            },
            Event::Candidate => {
                match req.target(){
                    Some(ref target) => match registry.borrow().get(target){
                        Some(target_conn) => {
                            let mut obj = req.to_json().as_object_mut().unwrap().clone();
                            obj.insert("from".to_string(), from.to_hex().to_json() );

                            target_conn.send(Json::Object(obj.clone()).to_string());
                            Ok(Response::Success(Some(req.id()), Json::String("ok".to_string())))
                        },
                        None              => {
                            let mut e = BTreeMap::new();
                            e.insert("code".to_string(),    404usize.to_json() );
                            e.insert("message".to_string(), "目标不存在".to_json() );
                            Ok(Response::Error(Some(req.id()), Json::Object(e)))
                        }
                    },
                    None => {
                        let mut e = BTreeMap::new();
                        e.insert("code".to_string(),    400usize.to_json() );
                        e.insert("message".to_string(), "未指定目标".to_json() );
                        Ok(Response::Error(Some(req.id()), Json::Object(e)))
                    }
                }
            }
        }
    }
}
