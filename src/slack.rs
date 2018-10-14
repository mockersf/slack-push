#[derive(Serialize, Deserialize, Debug)]
pub struct Channel {
    id: String,
    name: String,
    created: u64,
    creator: Option<String>,
}
