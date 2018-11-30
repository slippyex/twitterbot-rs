use crate::models::BotTweet;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Retweeted {
    pub tweets: Vec<BotTweet>,
}
