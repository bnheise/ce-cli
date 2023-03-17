use headless_common::reqwest;

pub struct LiferayClientOptions {
    pub base_url: reqwest::Url,
    pub username: String,
    pub password: String,
}
