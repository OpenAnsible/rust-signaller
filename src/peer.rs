
use std::str::FromStr;
use std::string::ToString;
use ::{
    BTreeMap, SocketAddr,
    ObjectId, Token, Sender, 
    Json, ToJson, ToHex, Object
};

#[derive(Debug)]
pub struct Peer {
    id   : ObjectId,
    token: Token,
    name : String,
    info : Option<String>,
    addr : Option<SocketAddr>,
    conn : Sender,
    ctime: f64,
    utime: f64
}

#[derive(Debug)]
pub struct Peers {
    map: BTreeMap<String, Peer>
}


impl ToString for ObjectId {
    fn to_string(&self) -> String {
        self.to_hex()
    }
}
impl ToJson for ObjectId {
    fn to_json(&self) -> Json {
        self.to_string().to_json()
    }
}

impl ToJson for Token {
    fn to_json(&self) -> Json {
        self.as_usize().to_json()
    }
}
impl ToJson for SocketAddr {
    fn to_json(&self) -> Json {
        self.to_string().to_json()
    }
}


impl ToJson for Peer {
    fn to_json(&self) -> Json {
        let mut json = BTreeMap::new();
        json.insert("id".to_string(),      self.id.to_json()    );
        // json.insert("token".to_string(),   self.token.to_json() );
        json.insert("name".to_string(),    self.name.to_json()  );
        json.insert("info".to_string(),    self.info.to_json()  );
        // json.insert("addr".to_string(),    self.addr.to_json()  );
        // json.insert("conn".to_string(),    self.conn.to_json()  );
        json.insert("ctime".to_string(),   self.ctime.to_json() );
        json.insert("utime".to_string(),   self.utime.to_json() );
        Json::Object(json)
    }
}
impl ToString for Peer {
    fn to_string(&self) -> String {
        self.to_json().to_string()
    }
}
impl Peer{
    pub fn new(token: Token, name: String, info: Option<String>,
               addr: Option<SocketAddr>, conn: Sender, ctime: f64, utime: f64) 
               -> Result<Peer, ()> {

        let id = ObjectId::new();
        Ok(Peer {
            id   : id,
            token: token,
            name : name,
            info : info,
            addr : addr,
            conn : conn,
            ctime: ctime,
            utime: utime
        })
    }
    pub fn id(&self) -> ObjectId {
        self.id
    }
    pub fn token(&self) -> Token{
        self.token
    }
    pub fn name(&self) -> String {
        self.name
    }
    pub fn info(&self) -> Option<String> {
        self.info
    }
    pub fn addr(&self) -> Option<SocketAddr> {
        self.addr
    }
    pub fn conn(&self) -> Sender {
        self.conn
    }
    pub fn ctime(&self) -> f64 {
        self.ctime
    }
    pub fn utime(&self) -> f64 {
        self.utime
    }

}


impl ToJson for Peers {
    fn to_json(&self) -> Json {
        self.map.to_json()
    }
}
impl ToString for Peers {
    fn to_string(&self) -> String {
        self.to_json().to_string()
    }
}

impl Peers {
    pub fn new() -> Result<Peers> {
        Ok(Peers::empty())
    }
    pub fn empty() -> Peers {
        let map: BTreeMap<String, Peer> = BTreeMap::new();
        Peers { map: map }
    }
    pub fn get(&self, oid: &str) -> Option<&Peer> {
        self.map.get(oid)
    }
    pub fn contains_key(&self, oid: &str) -> bool {
        self.map.contains_key(oid)
    }
    pub fn insert(&self, peer: Peer) -> Option<Peer> {
        let oid = peer.id().to_string();
        // If the map did not have this key present, None is returned.
        // If the map did have this key present, the value is updated, 
        // and the old value is returned. The key is not updated
        self.map.insert(oid, peer)
    }
    pub fn remove(&self, oid: &str) -> Option<Peer> {
        // If the map did not have this key present, None is returned.
        // If the map did have this key present, Remove the key and value,  
        // and the old value is returned.
        self.map.remove(oid)
    }
    pub fn keys(&self) -> Vec<String> {
        let keys: Vec<String> = self.map.keys().cloned().collect();
        keys
    }
    pub fn values(&self) -> Vec<Peer> {
        let values: Vec<Peer> = self.map.values().cloned().collect();
        values
    }
    pub fn to_vec(&self) -> Vec<Peer> {
        self.values()
    }
}

