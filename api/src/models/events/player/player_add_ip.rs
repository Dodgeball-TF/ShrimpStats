use crate::Event;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Inner {
    pub uuid: String,
    pub ip: String,
}

impl Event for Inner {
    fn handle(&self) {
        println!("PlayerAddIp: {self:?}");
    }
}
