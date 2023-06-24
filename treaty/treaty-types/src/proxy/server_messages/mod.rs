pub mod http;

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
pub struct RegisterLoginRequest {
    pub login: String,
    pub pw: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
pub struct RegisterLoginReply {
    pub is_successful: bool,
    pub host_id: Option<String>,
    pub error: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
pub struct AuthForTokenRequest {
    pub login: String,
    pub pw: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
pub struct AuthForTokenReply {
    pub is_successful: bool,
    pub expiration_utc: Option<String>,
    pub jwt: Option<String>,
    pub id: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
pub struct ExecuteRequest {
    pub login: Option<String>,
    pub pw: Option<String>,
    pub jwt: Option<String>,
    pub request_type: u16,
    pub request_json: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
pub struct ExecuteReply {
    pub login_success: bool,
    pub execute_success: bool,
    pub reply: Option<String>,
}
