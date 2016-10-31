//! Rust wrapper for the zKillboard API.

extern crate hyper;
use hyper::Client;
use hyper::client::response::Response;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic_request() {
        extern crate hyper;
        use hyper::Client;

        let client = Client::new();
        let response = client.get("https://zkillboard.com/api/kills/characterID/268946627/")
                             .send()
                             .unwrap();
    }
}
