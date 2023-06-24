use crate::{
    db_interface::dbi_actions::DbiActions,
    error::TreatyDbError,
    models::{CdsHosts, DataInfo},
    query_parser::get_table_name,
    remote::remote_actions::RemoteActions,
    treaty_proto::ExecuteWriteRequest,
};
use tracing::{info, warn};
use treaty_types::enums::*;

#[derive(Debug, Clone)]
pub struct IoResult {
    pub is_successful: bool,
    pub rows_affected: u32,
}

pub async fn handle_delete_write_at_participant<T: DbiActions + Clone, R: RemoteActions + Clone>(
    db: &T,
    remote: &R,
    request: &ExecuteWriteRequest,
    known_host: &CdsHosts,
) -> Result<IoResult, TreatyDbError> {
    let db_type = db.db_type();
    let db_name = &request.database_name;
    let statement = &request.sql_statement;
    let where_clause = &request.where_clause;
    let table_name = get_table_name(statement, db_type);

    let delete_behavior = db
        .get_deletes_to_host_behavior(db_name, &table_name)
        .unwrap_or(DeletesToHostBehavior::DoNothing);

    let result = db.delete_data_in_partial_db(
        db_name,
        &table_name,
        statement,
        where_clause,
        &known_host.host_id,
    );

    match result {
        Ok(delete_result) => match delete_behavior {
            DeletesToHostBehavior::Unknown => todo!(),
            DeletesToHostBehavior::SendNotification => {
                let remote_host = db.get_cds_host_for_part_db(db_name).unwrap().unwrap();
                let own_host_info = db
                    .treaty_get_host_info()
                    .expect("no host info is set")
                    .unwrap();

                let notify_result = remote
                    .notify_host_of_removed_row(
                        &remote_host,
                        &own_host_info,
                        db_name,
                        &table_name,
                        delete_result.row_id,
                    )
                    .await;

                if !notify_result {
                    warn!("notify host {remote_host:?} of delete was not successful");
                }

                if delete_result.is_successful && notify_result {
                    return Ok(IoResult {
                        is_successful: true,
                        rows_affected: 1,
                    });
                } else {
                    return Ok(IoResult {
                        is_successful: false,
                        rows_affected: 0,
                    });
                }
            }
            DeletesToHostBehavior::DoNothing => {
                info!("configured to not notify host on local delete");
                if delete_result.is_successful {
                    return Ok(IoResult {
                        is_successful: true,
                        rows_affected: 1,
                    });
                }
            }
        },
        Err(e) => return Err(e),
    }

    todo!()
}

pub async fn handle_update_write_at_participant<T: DbiActions + Clone, R: RemoteActions + Clone>(
    db: &T,
    remote: &R,
    request: &ExecuteWriteRequest,
    known_host: &CdsHosts,
) -> Result<IoResult, TreatyDbError> {
    let db_type = db.db_type();
    let db_name = &request.database_name;
    let statement = &request.sql_statement;
    let where_clause = &request.where_clause;
    let table_name = get_table_name(statement, db_type);

    let update_behavior = db
        .get_updates_to_host_behavior(db_name, &table_name)
        .unwrap_or(UpdatesToHostBehavior::DoNothing);

    let result = db.update_data_into_partial_db(
        db_name,
        &table_name,
        statement,
        &known_host.host_id,
        where_clause,
    );

    match result {
        Ok(data_result) => {
            let data_info = DataInfo {
                db_name: db_name.clone(),
                table_name: table_name.clone(),
                row_id: data_result.row_id,
                hash: data_result.data_hash,
                is_deleted: false,
            };

            match update_behavior {
                UpdatesToHostBehavior::Unknown => todo!(),
                UpdatesToHostBehavior::SendDataHashChange => {
                    let result = db.get_cds_host_for_part_db(db_name);

                    match result {
                        Ok(remote_host) => {
                            let own_host_info = db
                                .treaty_get_host_info()
                                .expect("no host info is set")
                                .unwrap();

                            let notify_result = remote
                                .notify_host_of_updated_hash(
                                    &remote_host.unwrap(),
                                    &own_host_info,
                                    &data_info,
                                )
                                .await;

                            if data_result.is_successful && notify_result {
                                Ok(IoResult {
                                    is_successful: true,
                                    rows_affected: 1,
                                })
                            } else {
                                Ok(IoResult {
                                    is_successful: false,
                                    rows_affected: 0,
                                })
                            }
                        }
                        Err(e) => Err(e),
                    }
                }
                UpdatesToHostBehavior::DoNothing => Ok(IoResult {
                    is_successful: true,
                    rows_affected: 1,
                }),
            }
        }
        Err(e) => Err(e),
    }
}
