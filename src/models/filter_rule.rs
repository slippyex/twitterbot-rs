
use crate::models::FilterQuery;

#[derive(Serialize, Deserialize, Clone)]
pub struct FilterRule {
    pub id: Option<u8>,
    pub query: FilterQuery,
    pub retweet: Option<bool>,
}
