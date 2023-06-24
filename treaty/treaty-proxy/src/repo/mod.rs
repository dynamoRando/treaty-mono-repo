pub mod sqlite_repo;

use chrono::{DateTime, Utc};

use crate::{user_info::UserInfo, TreatyProxyErr};

pub trait RepoActions {
    fn config(&self);
    fn register_user(&self, un: &str, hash: &str) -> Result<(), TreatyProxyErr>;
    fn save_token(
        &self,
        login: &str,
        token: &str,
        expiration: DateTime<Utc>,
    ) -> Result<(), TreatyProxyErr>;
    fn revoke_tokens_for_login(&self, un: &str) -> bool;
    fn login_has_token(&self, un: &str) -> bool;
    fn get_user_with_token(&self, token: &str) -> Result<UserInfo, TreatyProxyErr>;
    fn verify_token(&self, token: &str) -> bool;
    fn delete_expired_tokens(&self);
    fn has_user(&self, un: &str) -> bool;
    fn has_host(&self, id: &str) -> bool;
    fn get_host(&self, id: &str) -> Result<UserInfo, TreatyProxyErr>;
    fn get_user(&self, un: &str) -> Result<UserInfo, TreatyProxyErr>;
    fn update_user(&self, u: &UserInfo) -> Result<(), TreatyProxyErr>;
}
