use std::pin::Pin;

use stdext::function_name;
use tracing::trace;
use treaty_types::enums::{CommunicationType, DatabaseType};

use crate::{
    data_service_handler::{
        data_service_handler_actions::DataServiceHandlerActions, DataServiceHandler,
    },
    db_interface::{dbi::DbInterface, postgres::PostgresDbi, sqlite::SqliteDbi},
    info_service_handler::{
        info_service_handler_actions::InfoServiceHandlerActions, InfoServiceHandler,
    },
    remote::{remote_grpc::RemoteGrpc, remote_http::RemoteHttp, Remote},
    settings::Settings,
    user_service_handler::{
        user_service_handler_actions::UserServiceHandlerActions, UserServiceHandler,
    },
};

pub struct HandlerBuilder {}

impl HandlerBuilder {
    pub async fn build_user_pin(
        dir: Option<&str>,
        settings: &Settings,
        communication_type: CommunicationType,
    ) -> Pin<Box<dyn UserServiceHandlerActions + Send + Sync>> {
        trace!("[{}]: Building new user handler...", function_name!());

        match communication_type {
            CommunicationType::Grpc => {
                let database_type = settings.database_type;

                trace!("[{}]: Init db..", function_name!());

                match database_type {
                    DatabaseType::Unknown => todo!(),
                    DatabaseType::Sqlite => {
                        let sqlite = SqliteDbi::new(
                            settings.backing_database_name.as_ref().unwrap(),
                            dir.unwrap(),
                        );
                        let interface = DbInterface::<SqliteDbi>::new(sqlite);
                        interface.init(&settings.admin_un, &settings.admin_pw).await;

                        let grpc: RemoteGrpc = match &settings.tls_options {
                            Some(tls) => RemoteGrpc::new(
                                &settings.grpc_data_service_addr_port,
                                &settings.grpc_info_service_addr_port,
                                settings.data_grpc_timeout_in_seconds,
                                Some(tls.clone().into()),
                            ),
                            None => RemoteGrpc::new(
                                &settings.grpc_data_service_addr_port,
                                &settings.grpc_info_service_addr_port,
                                settings.data_grpc_timeout_in_seconds,
                                None,
                            ),
                        };

                        let remote = Remote::new(grpc);
                        let user: UserServiceHandler<DbInterface<SqliteDbi>, Remote<RemoteGrpc>> =
                            UserServiceHandler::new(interface, remote, settings.clone());
                        return Box::pin(user);
                    }

                    DatabaseType::Postgres => todo!("postgres"),
                }
            }
            CommunicationType::Http => {
                let database_type = settings.database_type;

                trace!("[{}]: Init db..", function_name!());

                match database_type {
                    DatabaseType::Unknown => todo!(),
                    DatabaseType::Sqlite => {
                        let sqlite = SqliteDbi::new(
                            settings.backing_database_name.as_ref().unwrap(),
                            dir.unwrap(),
                        );
                        let interface = DbInterface::<SqliteDbi>::new(sqlite);
                        interface.init(&settings.admin_un, &settings.admin_pw).await;
                        let http = RemoteHttp::new(
                            &settings.http_addr,
                            settings.http_port as u32,
                            settings.use_tls,
                            settings.tls_http_options.clone(),
                        );
                        let remote = Remote::new(http);
                        let user: UserServiceHandler<DbInterface<SqliteDbi>, Remote<RemoteHttp>> =
                            UserServiceHandler::new(interface, remote, settings.clone());
                        return Box::pin(user);
                    }

                    DatabaseType::Postgres => {
                        let postgres_settings = &settings.postgres_settings.as_ref().unwrap();
                        let postgres_dbi = PostgresDbi::new(
                            &postgres_settings.host,
                            postgres_settings.port,
                            &postgres_settings.un,
                            &postgres_settings.pw,
                            &postgres_settings.schema_name,
                            &postgres_settings.db_name,
                            postgres_settings.use_treaty_schema,
                        );

                        let interface = DbInterface::<PostgresDbi>::new(postgres_dbi);
                        interface
                            .init(&postgres_settings.un, &postgres_settings.pw)
                            .await;

                        trace!(
                            "[{}]: Init complete. Creating new RemoteHttp",
                            function_name!()
                        );

                        let http = RemoteHttp::new(
                            &settings.http_addr,
                            settings.http_port as u32,
                            settings.use_tls,
                            settings.tls_http_options.clone(),
                        );
                        let remote = Remote::new(http);

                        trace!("[{}]: Creating new UserServiceHandler", function_name!());

                        let user: UserServiceHandler<DbInterface<PostgresDbi>, Remote<RemoteHttp>> =
                            UserServiceHandler::new(interface, remote, settings.clone());

                        trace!("[{}]: Returning UserServiceHandler", function_name!());

                        return Box::pin(user);
                    }
                }
            }
        }
    }

    pub async fn build_data_pin(
        dir: Option<&str>,
        settings: &Settings,
    ) -> Pin<Box<dyn DataServiceHandlerActions + Send + Sync>> {
        trace!("[{}]: Building new data handler...", function_name!());

        let database_type = settings.database_type;

        trace!("[{}]: Init db..", function_name!());

        match database_type {
            DatabaseType::Unknown => todo!(),
            DatabaseType::Sqlite => {
                let sqlite = SqliteDbi::new(
                    settings.backing_database_name.as_ref().unwrap(),
                    dir.unwrap(),
                );
                let interface = DbInterface::<SqliteDbi>::new(sqlite);
                interface.init(&settings.admin_un, &settings.admin_pw).await;
                Box::pin(DataServiceHandler::new(interface, settings.clone()))
            }

            DatabaseType::Postgres => {
                let postgres_settings = &settings.postgres_settings.as_ref().unwrap();
                let postgres_dbi = PostgresDbi::new(
                    &postgres_settings.host,
                    postgres_settings.port,
                    &postgres_settings.un,
                    &postgres_settings.pw,
                    &postgres_settings.schema_name,
                    &postgres_settings.db_name,
                    postgres_settings.use_treaty_schema,
                );

                let interface = DbInterface::<PostgresDbi>::new(postgres_dbi);
                interface
                    .init(&postgres_settings.un, &postgres_settings.pw)
                    .await;
                Box::pin(DataServiceHandler::new(interface, settings.clone()))
            }
        }
    }

    pub async fn build_info_pin(
        dir: Option<&str>,
        settings: &Settings,
    ) -> Pin<Box<dyn InfoServiceHandlerActions + Send + Sync>> {
        trace!("[{}]: Building new info handler...", function_name!());

        let database_type = settings.database_type;

        trace!("[{}]: Init db..", function_name!());

        match database_type {
            DatabaseType::Unknown => todo!(),
            DatabaseType::Sqlite => {
                let sqlite = SqliteDbi::new(
                    settings.backing_database_name.as_ref().unwrap(),
                    dir.unwrap(),
                );
                let interface = DbInterface::<SqliteDbi>::new(sqlite);
                interface.init(&settings.admin_un, &settings.admin_pw).await;
                Box::pin(InfoServiceHandler::new(interface, settings.clone()))
            }

            DatabaseType::Postgres => {
                let postgres_settings = &settings.postgres_settings.as_ref().unwrap();
                let postgres_dbi = PostgresDbi::new(
                    &postgres_settings.host,
                    postgres_settings.port,
                    &postgres_settings.un,
                    &postgres_settings.pw,
                    &postgres_settings.schema_name,
                    &postgres_settings.db_name,
                    postgres_settings.use_treaty_schema,
                );

                let interface = DbInterface::<PostgresDbi>::new(postgres_dbi);
                interface
                    .init(&postgres_settings.un, &postgres_settings.pw)
                    .await;
                Box::pin(InfoServiceHandler::new(interface, settings.clone()))
            }
        }
    }

    pub async fn build_data(
        dir: Option<&str>,
        settings: &Settings,
    ) -> Box<dyn DataServiceHandlerActions + Send + Sync> {
        trace!("[{}]: Building new data handler...", function_name!());

        let database_type = settings.database_type;

        trace!("[{}]: Init db..", function_name!());

        match database_type {
            DatabaseType::Unknown => todo!(),
            DatabaseType::Sqlite => {
                let sqlite = SqliteDbi::new(
                    settings.backing_database_name.as_ref().unwrap(),
                    dir.unwrap(),
                );
                let interface = DbInterface::<SqliteDbi>::new(sqlite);
                interface.init(&settings.admin_un, &settings.admin_pw).await;
                Box::new(DataServiceHandler::new(interface, settings.clone()))
            }

            DatabaseType::Postgres => {
                let postgres_settings = &settings.postgres_settings.as_ref().unwrap();
                let postgres_dbi = PostgresDbi::new(
                    &postgres_settings.host,
                    postgres_settings.port,
                    &postgres_settings.un,
                    &postgres_settings.pw,
                    &postgres_settings.schema_name,
                    &postgres_settings.db_name,
                    postgres_settings.use_treaty_schema,
                );

                let interface = DbInterface::<PostgresDbi>::new(postgres_dbi);
                interface
                    .init(&postgres_settings.un, &postgres_settings.pw)
                    .await;
                Box::new(DataServiceHandler::new(interface, settings.clone()))
            }
        }
    }

    pub async fn build_info(
        dir: Option<&str>,
        settings: &Settings,
    ) -> Box<dyn InfoServiceHandlerActions + Send + Sync> {
        trace!("[{}]: Building new info handler...", function_name!());

        let database_type = settings.database_type;

        trace!("[{}]: Init db..", function_name!());

        match database_type {
            DatabaseType::Unknown => todo!(),
            DatabaseType::Sqlite => {
                let sqlite = SqliteDbi::new(
                    settings.backing_database_name.as_ref().unwrap(),
                    dir.unwrap(),
                );
                let interface = DbInterface::<SqliteDbi>::new(sqlite);
                interface.init(&settings.admin_un, &settings.admin_pw).await;
                Box::new(InfoServiceHandler::new(interface, settings.clone()))
            }

            DatabaseType::Postgres => {
                let postgres_settings = &settings.postgres_settings.as_ref().unwrap();
                let postgres_dbi = PostgresDbi::new(
                    &postgres_settings.host,
                    postgres_settings.port,
                    &postgres_settings.un,
                    &postgres_settings.pw,
                    &postgres_settings.schema_name,
                    &postgres_settings.db_name,
                    postgres_settings.use_treaty_schema,
                );

                let interface = DbInterface::<PostgresDbi>::new(postgres_dbi);
                interface
                    .init(&postgres_settings.un, &postgres_settings.pw)
                    .await;
                Box::new(InfoServiceHandler::new(interface, settings.clone()))
            }
        }
    }

    pub async fn build_user(
        dir: Option<&str>,
        settings: &Settings,
        communication_type: CommunicationType,
    ) -> Box<dyn UserServiceHandlerActions + Send + Sync> {
        trace!("[{}]: Building new user handler...", function_name!());

        match communication_type {
            CommunicationType::Grpc => {
                let database_type = settings.database_type;

                trace!("[{}]: Init db..", function_name!());

                match database_type {
                    DatabaseType::Unknown => todo!(),
                    DatabaseType::Sqlite => {
                        let sqlite = SqliteDbi::new(
                            settings.backing_database_name.as_ref().unwrap(),
                            dir.unwrap(),
                        );
                        let interface = DbInterface::<SqliteDbi>::new(sqlite);
                        interface.init(&settings.admin_un, &settings.admin_pw).await;

                        let grpc: RemoteGrpc = match &settings.tls_options {
                            Some(tls) => RemoteGrpc::new(
                                &settings.grpc_data_service_addr_port,
                                &settings.grpc_info_service_addr_port,
                                settings.data_grpc_timeout_in_seconds,
                                Some(tls.clone().into()),
                            ),
                            None => RemoteGrpc::new(
                                &settings.grpc_data_service_addr_port,
                                &settings.grpc_info_service_addr_port,
                                settings.data_grpc_timeout_in_seconds,
                                None,
                            ),
                        };
                        let remote = Remote::new(grpc);
                        let user: UserServiceHandler<DbInterface<SqliteDbi>, Remote<RemoteGrpc>> =
                            UserServiceHandler::new(interface, remote, settings.clone());
                        return Box::new(user);
                    }

                    DatabaseType::Postgres => todo!("postgres"),
                }
            }
            CommunicationType::Http => {
                let database_type = settings.database_type;

                trace!("[{}]: Init db..", function_name!());

                match database_type {
                    DatabaseType::Unknown => todo!(),
                    DatabaseType::Sqlite => {
                        let sqlite = SqliteDbi::new(
                            settings.backing_database_name.as_ref().unwrap(),
                            dir.unwrap(),
                        );
                        let interface = DbInterface::<SqliteDbi>::new(sqlite);
                        interface.init(&settings.admin_un, &settings.admin_pw).await;
                        let http = RemoteHttp::new(
                            &settings.http_addr,
                            settings.http_port as u32,
                            settings.use_tls,
                            settings.tls_http_options.clone(),
                        );
                        let remote = Remote::new(http);
                        let user: UserServiceHandler<DbInterface<SqliteDbi>, Remote<RemoteHttp>> =
                            UserServiceHandler::new(interface, remote, settings.clone());
                        return Box::new(user);
                    }

                    DatabaseType::Postgres => {
                        let postgres_settings = &settings.postgres_settings.as_ref().unwrap();
                        let postgres_dbi = PostgresDbi::new(
                            &postgres_settings.host,
                            postgres_settings.port,
                            &postgres_settings.un,
                            &postgres_settings.pw,
                            &postgres_settings.schema_name,
                            &postgres_settings.db_name,
                            postgres_settings.use_treaty_schema,
                        );

                        let interface = DbInterface::<PostgresDbi>::new(postgres_dbi);
                        interface
                            .init(&postgres_settings.un, &postgres_settings.pw)
                            .await;

                        trace!(
                            "[{}]: Init complete. Creating new RemoteHttp",
                            function_name!()
                        );

                        let http = RemoteHttp::new(
                            &settings.http_addr,
                            settings.http_port as u32,
                            settings.use_tls,
                            settings.tls_http_options.clone(),
                        );
                        let remote = Remote::new(http);

                        trace!("[{}]: Creating new UserServiceHandler", function_name!());

                        let user: UserServiceHandler<DbInterface<PostgresDbi>, Remote<RemoteHttp>> =
                            UserServiceHandler::new(interface, remote, settings.clone());

                        trace!("[{}]: Returning UserServiceHandler", function_name!());

                        return Box::new(user);
                    }
                }
            }
        }
    }
}
