use treaty_types::types::treaty_proto::AuthRequestWebToken;

/// Represents a JWT granted from an Treaty or Treaty-PROXY instance
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Token {
    /// The JWT
    pub jwt: String,
    /// The UTC expiration time of the token
    pub jwt_exp: String,
    /// The HTTP addr of the Treaty instance the token is for
    pub addr: String,
    /// if we are logged in or not
    pub is_logged_in: bool,
    /// the host id
    pub id: Option<String>,
}

impl Default for Token {
    fn default() -> Self {
        Self::new()
    }
}

impl Token {
    pub fn new() -> Token {
        Token {
            jwt: "".to_string(),
            jwt_exp: "".to_string(),
            addr: "".to_string(),
            is_logged_in: false,
            id: None,
        }
    }

    /// Returns an AuthRequest in JSON format with this JWT as the authentication method
    pub fn auth_json(&self) -> String {
        let request = self.auth();
        serde_json::to_string(&request).unwrap()
    }

    /// Returns an AuthRequest with this JWT as the authentication method
    pub fn auth(&self) -> AuthRequestWebToken {
        AuthRequestWebToken {
            jwt: self.jwt.clone(),
        }
    }
}
