use crate::{db_interface::dbi_actions::DbiActions, settings::Settings};

// natively handle the gRPC implementation of the service
// this module just passses call from the gRPC implementation back to this module
pub(crate) mod grpc;

// definitions for the info service
pub mod info_service_handler_actions;

#[derive(Debug, Clone)]
pub struct InfoServiceHandler<D: DbiActions + Clone + Send + Sync + 'static> {
    db: D,
    settings: Settings,
}
