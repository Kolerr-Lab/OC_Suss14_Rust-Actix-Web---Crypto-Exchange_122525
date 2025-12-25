use std::env;

fn get_api_key() -> String {
    env::var("API_KEY").expect("API_KEY must be set")
}