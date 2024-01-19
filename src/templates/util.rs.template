use std::sync::Mutex;

pub struct ThanixClient {
    pub client: reqwest::blocking::Client,
    pub base_url: String,
    pub authentication_token: String,
}

pub static THANIX_CLIENT: Mutex<Option<ThanixClient>> = Mutex::new(None);
