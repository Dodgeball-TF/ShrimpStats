use macro_utils::UnclassifiedEvent;

pub trait Event {
    fn handle(&self);
}

#[UnclassifiedEvent]
pub enum UnclassifiedEvent {
    ServerAmendActMap,
    ServerAmendDeaths,
    ServerAmendIp,
    ServerAmendKills,
    ServerAmendMaxPlayers,
    ServerAmendName,
    ServerAmendPlayers,
    ServerAmendPort,
    ServerAmendRconPassword,
    ServerDelete,
    ServerRegister,
    PlayerRegister,
    PlayerDelete,
    PlayerAmendName,
    PlayerAddIp,
}

impl UnclassifiedEvent {
    pub fn from_log_line(raw: &str) -> Self {
        todo!("Implement UnclassifiedEvent::from_log_line");
    }

    pub async fn from_extension(raw: &str) -> Self {
        // dummy sleep to make the compiler happy
        tokio::time::sleep(tokio::time::Duration::from_secs(0)).await;
        todo!("Implement UnclassifiedEvent::from_extension");
    }
}
