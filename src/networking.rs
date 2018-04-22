use std::net::TcpStream;
use std::io::{Read, Write};
use std::io::Error;

use serde_json;

use data::Game;

pub fn get_cache() -> Result<String, Error> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")
        .expect("Error binding to ip");
    stream.write_all(b"\"GetCache\"\n")
        .expect("Error sending request");
    let mut output = String::new();
    stream.read_to_string(&mut output)
        .expect("Error reading response");
    Ok(output)
}

pub fn update_game(game: Game) -> Result<(), Error> {
    use parse;
    let mut stream = TcpStream::connect("127.0.0.1:8080")
        .expect("Error binding to ip");
    let request = parse::game(game)
        .expect("Error parsing the game struct into a String");
    stream.write_all(request.as_bytes())?;
//    stream.write_all("{\"UpdateGame\":{\"name\":\"A Game\",\"author\":\"Hiruna\",\"join_code\":\"00x0\"}}".as_bytes())?;
    Ok(())
}
