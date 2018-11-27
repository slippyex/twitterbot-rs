use crate::models::FollowerRange;

#[derive(Serialize, Deserialize)]
pub struct FollowStrategy {
    #[serde(rename = "type")]
    pub type_: String,
    pub count: u8,
    pub follower_range: FollowerRange
}
