use crate::Event;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Inner {
    pub uuid: String,
    pub name: String,
    pub ips: Vec<String>,
    pub server_id: String,
    pub country: String,
}

impl Event for Inner {
    fn handle(&self) {
        println!("ServerRegister: {self:?}");
    }
}
