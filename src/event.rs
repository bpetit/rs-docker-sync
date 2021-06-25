use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Event {
    pub Type: String,
    pub Action: String,
    pub status: String,
    pub id: String,
    pub Actor: Actor,
    pub scope: String,
    pub time: String,
    pub timeNano: Option<String>,
    pub Experimental: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Actor {
    pub ID: String,
    pub Attributes: HashMap<String, String>,
}
