#[derive(Serialize, Deserialize, Clone)]
pub struct BotConfig {
    pub user_id: u64,
    pub screen_name: String,
    pub twitter_consumer_key: String,
    pub twitter_consumer_secret: String,
    pub twitter_access_key: String,
    pub twitter_token_secret: String,
    pub bot_port: u16
}

impl Default for BotConfig {
    fn default () -> BotConfig {
        BotConfig {
            user_id: 0,
            screen_name: String::new(),
            twitter_access_key: String::new(),
            twitter_token_secret: String::new(),
            twitter_consumer_secret: String::new(),
            twitter_consumer_key: String::new(),
            bot_port: 8000
        }
    }
}
