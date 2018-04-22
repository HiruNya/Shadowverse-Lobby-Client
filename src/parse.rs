use serde_json::{to_string, from_str, Error};
use data::{Game, Request};

pub fn cache(text: String) -> Result<Vec<Game>, Error> {
    from_str(text.as_str())
}

pub fn game(g: Game) -> Result<String, Error> {
    let request = Request::UpdateGame(g);
    let mut text = to_string(&request)?;
    text.push('\n');
    Ok(text)
}
