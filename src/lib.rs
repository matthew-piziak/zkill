//! Rust wrapper for the zKillboard API.

extern crate hyper;
use hyper::Client;

extern crate rustc_serialize;
use rustc_serialize::json::Json;

// filter out item flag 5 for cargo
// filter out item flag 87 for drones
// filter out ship ID 670 for pods

pub fn sound_losses() {
    use std::io::prelude::*;

    let client = Client::new();
    let endpoint = endpoint(false, true, 99000739);
    let mut response = client.get(&endpoint).send().unwrap();

    let mut response_string = String::new();
    response.read_to_string(&mut response_string).unwrap();
    let data = Json::from_str(&response_string).unwrap();
    let kills = data.as_array().unwrap();
    println!("First kill: {}", kills.first().unwrap());
}

fn endpoint(include_kills: bool, include_losses: bool, alliance_id: usize) -> String {
    if !include_kills && !include_losses {
        panic!("No results will be returned.")
    }
    format!("https://zkillboard.com/api/allianceID/{}{}{}",
            alliance_id,
            if include_kills {
                "/kills"
            } else {
                ""
            },
            if include_losses {
                "/losses"
            } else {
                ""
            })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sound_losses() {
        sound_losses();
        assert_eq!(false, true);
    }
}
