use regex::Regex;

pub struct ThanixClient {
    pub client: reqwest::blocking::Client,
    pub base_url: String,
    pub authentication_token: String,
}

pub fn remove_square_braces(s: &str) -> String {
    let re = Regex::new(r"\[\d+\]").unwrap();

    re.replace_all(s, "").to_string()
}
