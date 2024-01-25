use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Player {
    pub uuid: String,
    pub name: String,
    pub ips: Vec<String>,
    pub server_id: String,
    pub country: String,
}
