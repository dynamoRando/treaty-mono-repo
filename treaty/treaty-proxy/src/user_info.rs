#[derive(Debug, Clone)]
pub struct UserInfo {
    pub username: String,
    pub folder: Option<String>,
    pub id: Option<String>,
    pub hash: Vec<u8>,
}
