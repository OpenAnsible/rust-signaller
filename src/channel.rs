
use std::str::FromStr;
use std::string::ToString;
use ::{
    BTreeMap, ObjectId, 
    Json, ToJson, ToHex, Object,
    Peers, Peer
};

/*
    {
        // if not exists, will create one.
        "method": "join", 
        "token" : "im password",      // 创建频道时，`token` 暗号是可选项，如果设置，则加入的人必须出示 `token` 才可以加入。
        "name"  : "test_channel", 
        "descp" : "",
        ...
        "peer"  : {
            // peer data ...
        }
    }
    {
        "method"     : "join", 
        "token"      : "im password",  // 如果是加密频道，必须带上 `token` 暗号才可以加入。
        "channel_id" : Channel ID,
        "peer"  : {
            // peer data ...
        }
    }

    {
        "method"     : "leave", // leave | quit | exit
        "channel_id" : Channel ID,
        "message"    : "暂时离开会 ..."
    }
    {
        "method"     : "broadcast",
        "channel_id" : Channel ID,
        "message"    : "频道公开消息...",
    }
    {
        "method"     : "message",
        "channel_id" : Channel ID,
        "message"    : "频道公开消息..." 
    }
    {
        "method"  : "message",
        "peer_id" : Peer ID,
        "message" : "私聊消息..."      // 跨越所有频道
    }
*/

#[derive(Debug)]
pub struct Channel {
    id   : ObjectId,
    token: Option<String>,   // password
    name : Option<String>,
    descp: Option<String>,
    info : Option<String>,
    peers: Peers,
    uid  : ObjectId,         // creator ( peer.id )
    ctime: f64,
    utime: f64
    
}

#[derive(Debug)]
pub struct Channels {
    map: BTreeMap<String, Channel>
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

impl ToJson for Channel {
    fn to_json(&self) -> Json {
        let mut json = BTreeMap::new();
        json.insert("id".to_string(),      self.id.to_json()    );
        // Password.
        // json.insert("token".to_string(),   self.token.to_json() );
        json.insert("name".to_string(),    self.name.to_json()  );
        json.insert("descp".to_string(),   self.descp.to_json() );
        json.insert("info".to_string(),    self.info.to_json()  );
        json.insert("peers".to_string(),   self.peers.to_json() );
        json.insert("uid".to_string(),     self.uid.to_json()   );
        json.insert("ctime".to_string(),   self.ctime.to_json() );
        json.insert("utime".to_string(),   self.utime.to_json() );
        Json::Object(json)
    }
}

impl ToString for Channel {
    fn to_string(&self) -> String {
        self.to_json().to_string()
    }
}

impl Channel{
    pub fn new(token: Option<String>, name: Option<String>, descp: Option<String>, 
               info: Option<String>, uid: ObjectId, ctime: f64, utime: f64) 
               -> Result<Channel, ()> {

        let id = ObjectId::new();
        Ok(Peer {
            id   : id,
            token: token,
            name : name,
            descp: descp,
            info : info,
            uid  : uid,
            ctime: ctime,
            utime: utime
        })
    }
    pub fn id(&self) -> ObjectId {
        self.id
    }
    pub fn token(&self) -> Option<String> {
        self.token
    }
    pub fn name(&self) -> Option<String> {
        self.name
    }
    pub fn descp(&self) -> Option<String> {
        self.descp
    }
    pub fn info(&self) -> Option<String> {
        self.info
    }
    pub fn uid(&self) -> ObjectId {
        // creator ( peer.id )
        self.uid
    }
    pub fn ctime(&self) -> f64 {
        self.ctime
    }
    pub fn utime(&self) -> f64 {
        self.utime
    }

}


impl ToJson for Channels {
    fn to_json(&self) -> Json {
        self.map.to_json()
    }
}
impl ToString for Channels {
    fn to_string(&self) -> String {
        self.to_json().to_string()
    }
}

impl Channels {
    pub fn new() -> Result<Channels> {
        Ok(Channels::empty())
    }
    pub fn empty() -> Channels {
        let map: BTreeMap<String, Channel> = BTreeMap::new();
        Channels { map: map }
    }
    pub fn get(&self, oid: &str) -> Option<&Channel> {
        self.map.get(oid)
    }
    pub fn contains_key(&self, oid: &str) -> bool {
        self.map.contains_key(oid)
    }
    pub fn insert(&self, channel: Channel) -> Option<Channel> {
        let oid = channel.id().to_string();
        // If the map did not have this key present, None is returned.
        // If the map did have this key present, the value is updated, 
        // and the old value is returned. The key is not updated
        self.map.insert(oid, channel)
    }
    pub fn remove(&self, oid: &str) -> Option<Channel> {
        // If the map did not have this key present, None is returned.
        // If the map did have this key present, Remove the key and value,  
        // and the old value is returned.
        self.map.remove(oid)
    }
    pub fn keys(&self) -> Vec<String> {
        let keys: Vec<String> = self.map.keys().cloned().collect();
        keys
    }
    pub fn values(&self) -> Vec<Channels> {
        let values: Vec<Channels> = self.map.values().cloned().collect();
        values
    }
    pub fn to_vec(&self) -> Vec<Channels> {
        self.values()
    }
}

