use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Server {
    pub uuid: String,
    pub ip: String,
    pub port: u16,
    pub rcon_password: String,
    pub kills: u32,
    pub deaths: u32,
    pub players: u32,
    pub max_players: u32,
    pub act_map: String,
    pub name: Option<String>,
}
