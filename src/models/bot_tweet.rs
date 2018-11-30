#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BotTweet {
    pub user_id: u64,
    pub text: String,
    pub filter_involved: String,
}
