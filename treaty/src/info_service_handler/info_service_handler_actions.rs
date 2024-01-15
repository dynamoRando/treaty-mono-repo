use async_trait::async_trait;
use stdext::function_name;
use tracing::{error, trace};
use treaty_types::enums::ContractStatus;

use crate::{
    db_interface::dbi_actions::DbiActions,
    models::CoopDatabaseParticipant,
    treaty_proto::{
        AuthRequestBasic, AuthRequestWebToken, ParticipantAcceptsContractRequest,
        ParticipantAcceptsContractResult, SaveContractRequest, SaveContractResult, TokenReply,
        TryAuthResult,
    },
};

use super::InfoServiceHandler;

#[async_trait]
pub trait InfoServiceHandlerActions {
    async fn accept_contract(
        &self,
        request: ParticipantAcceptsContractRequest,
    ) -> ParticipantAcceptsContractResult;
    async fn save_contract(&self, request: SaveContractRequest) -> SaveContractResult;
    async fn try_auth_web_token(&self, request: AuthRequestWebToken) -> TryAuthResult;
    async fn auth_for_token(&self, request: AuthRequestBasic) -> TokenReply;
}

#[async_trait]
impl<D: DbiActions + Clone + Send + Sync> InfoServiceHandlerActions for InfoServiceHandler<D> {
    async fn accept_contract(
        &self,
        request: ParticipantAcceptsContractRequest,
    ) -> ParticipantAcceptsContractResult {
        let debug_message_info = &request.telemetry.as_ref().unwrap().clone();

        trace!("{debug_message_info:?}");
        trace!("{request:?}");

        let participant_message = request.participant.as_ref().unwrap().clone();

        let coop_db_participant: CoopDatabaseParticipant;

        let accepted_participant = self
            .db
            .get_participant_by_alias(
                &request.database_name,
                &request.participant.as_ref().unwrap().alias,
            )
            .await
            .unwrap();

        if accepted_participant.is_none() {
            let _participant = self
                .db
                .get_participant_by_id(
                    &request.database_name,
                    &request.participant.as_ref().unwrap().participant_guid,
                )
                .await
                .unwrap();

            if _participant.is_some() {
                trace!(
                    "found participant: {:?}",
                    _participant.as_ref().unwrap().clone()
                );
                coop_db_participant = _participant.unwrap();
            } else {
                error!("Could not find participant by alias or id, this should not happen.");
                coop_db_participant = CoopDatabaseParticipant::default();
            }
        } else {
            trace!(
                "found participant: {:?}",
                accepted_participant.as_ref().unwrap().clone()
            );
            coop_db_participant = accepted_participant.unwrap();
        }

        let is_successful = self
            .db
            .update_participant_accepts_contract(
                &request.database_name,
                coop_db_participant,
                participant_message,
                &request.contract_version_guid,
            )
            .await
            .unwrap();

        ParticipantAcceptsContractResult {
            contract_acceptance_is_acknowledged: is_successful,
            is_error: false,
            error: None,
        }
    }
    async fn save_contract(&self, request: SaveContractRequest) -> SaveContractResult {
        trace!("[{}]: {request:?}", function_name!());
        let contract = request.contract.unwrap();
        let result = self.db.save_contract(contract).await;
        match result {
            Ok(save_result) => {
                trace!("[{}]: {save_result:?}", function_name!());
                let status = ContractStatus::to_u32(save_result.contract_status);
                SaveContractResult {
                    is_saved: save_result.is_successful,
                    contract_status: status,
                    participant_info: save_result.participant_information,
                    is_error: false,
                    error: None,
                }
            }
            Err(e) => SaveContractResult {
                is_saved: false,
                contract_status: 0,
                participant_info: None,
                is_error: true,
                error: Some(e.into()),
            },
        }
    }

    async fn try_auth_web_token(&self, request: AuthRequestWebToken) -> TryAuthResult {
        trace!("[{}]: {request:?}", function_name!());
        let token = request.jwt;
        let result = self.db.verify_token(&token).await;
        match result {
            Ok(is_verified) => TryAuthResult {
                is_authenticated: is_verified,
            },
            Err(e) => {
                error!("[{}]: {e:?}", function_name!());
                TryAuthResult {
                    is_authenticated: false,
                }
            }
        }
    }

    async fn auth_for_token(&self, request: AuthRequestBasic) -> TokenReply {
        trace!("[{}]: {request:?}", function_name!());
        let result = self
            .db
            .auth_for_token(
                &request.user_name,
                &request.pw,
                Some(self.settings.jwt_valid_time_in_minutes),
            )
            .await;
        trace!("[{}]: {result:?}", function_name!());
        match result {
            Ok(token) => token,
            Err(e) => {
                error!("[{}]: {e:?}", function_name!());
                TokenReply {
                    is_successful: false,
                    expiration_utc: String::from(""),
                    jwt: String::from(""),
                }
            }
        }
    }
}
