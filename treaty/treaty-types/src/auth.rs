use crate::types::treaty_proto::{
    AuthRequestAuthor, AuthRequestBasic, AuthRequestBinary, AuthRequestMetadata,
    AuthRequestWebToken,
};

/// A grab bag for authentication purposes
#[derive(Clone)]
pub struct AuthRequestData {
    /// the headers in the authentication request
    pub headers: Vec<String>,
    pub basic: Option<AuthRequestBasic>,
    pub web_token: Option<AuthRequestWebToken>,
    pub binary: Option<AuthRequestBinary>,
    pub metadata: Option<AuthRequestMetadata>,
    pub author: Option<AuthRequestAuthor>,
}
