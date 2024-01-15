use crate::{
    models::Table, treaty_proto::StatementResultset, user_service_handler::io::db::ReadResult,
};

impl From<ReadResult> for StatementResultset {
    fn from(value: ReadResult) -> Self {
        Self {
            number_of_rows_affected: value.rows_affected as u64,
            rows: value.rows,
            error: None,
            warning: value.warning,
        }
    }
}

impl From<Table> for StatementResultset {
    fn from(value: Table) -> Self {
        Self {
            number_of_rows_affected: value.rows.len() as u64,
            rows: value.convert_to_protobuf(),
            error: None,
            warning: None,
        }
    }
}

/*

-- may need to break this out of treaty-proto and tell prost not to
-- implement the Debug trait for this.

impl fmt::Debug for AuthRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let has_pw = !self.pw.is_empty();

        let pw_print = if has_pw {
            "[REDACTED]"
        } else {
            "[NOT PROVIDED]"
        };

        let has_hash = !self.pw_hash.is_empty();

        let pw_hash_print = if has_hash {
            "[REDACTED]"
        } else {
            "[NOT PROVIDED]"
        };

        let has_token = !self.token.is_empty();

        let token_hash_print = if has_token {
            "[REDACTED]"
        } else {
            "[NOT PROVIDED]"
        };

        let has_jwt = !self.jwt.is_empty();

        let jwt_print = if has_jwt {
            "[REDACTED]"
        } else {
            "[NOT PROVIDED]"
        };

        f.debug_struct("AuthRequest")
            .field("user_name", &self.user_name)
            .field("pw", &pw_print)
            .field("pw_hash", &pw_hash_print)
            .field("token", &token_hash_print)
            .field("jwt", &jwt_print)
            .field("id", &self.id)
            .finish()
    }
}
 */
