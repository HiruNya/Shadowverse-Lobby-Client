#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize)]
pub enum Request {
    GetCache,
    UpdateGame(Game),
}

use std::cmp::PartialEq;

impl PartialEq for Game {
    fn eq(&self, other: &Game) -> bool {
        ((self.author == other.author)&&(self.name==other.name)&&(self.join_code==other.join_code))
    }
}
