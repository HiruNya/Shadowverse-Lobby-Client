#[derive(Serialize, Deserialize)]
pub struct Game {
    pub name: String,
    pub author: String,
    pub join_code: String,
}

pub enum MsgToController {
    Shutdown,
    GetCache,
    CacheReceived(Vec<Game>),
    UpdateGame(Game)
}

pub enum MsgToGui {
    PopulateList(Vec<Game>),
    UpdateRequestSent,
    Error(String),
}