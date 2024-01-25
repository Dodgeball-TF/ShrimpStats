use crate::Event;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Inner {
    pub uuid: String,
}

impl Event for Inner {
    fn handle(&self) {
        println!("PlayerDelete: {self:?}");
    }
}
