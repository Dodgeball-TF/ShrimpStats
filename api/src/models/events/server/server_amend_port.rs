use crate::Event;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Inner {
  pub uuid: String,
  pub port: u16,
}

impl Event for Inner {
    fn handle(&self) {
        println!("ServerAmendPort: {self:?}");
    }
}