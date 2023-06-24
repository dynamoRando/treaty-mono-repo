use crate::{
    db_interface::dbi_actions::DbiActions,
    treaty_proto::{AuthResult, RowInfo, TreatyError, UpdateDataRequest, UpdateDataResult},
};

use treaty_types::enums::{PartialDataStatus, UpdatesFromHostBehavior};

pub async fn handle_update_command_into_table<T: DbiActions + Clone>(
    db: &T,
    request: &UpdateDataRequest,
    auth: &AuthResult,
) -> UpdateDataResult {
    let db_name = &request.database_name;
    let table_name = &request.table_name;
    let where_clause = request.where_clause.clone();
    let cmd = &request.cmd;
    let mut update_status: u32 = 0;
    let mut rows: Vec<RowInfo> = Vec::new();
    let mut is_successful = false;
    let mut message = String::from("");
    let mut error: Option<TreatyError> = None;

    if auth.is_authenticated {
        let result = db.get_cds_host_for_part_db(db_name);
        match result {
            Ok(opt_known_host) => {
                if let Some(known_host) = opt_known_host {
                    let result = db.get_updates_from_host_behavior(db_name, table_name);
                    match result {
                        Ok(update_behavior) => {
                            match update_behavior {
                                UpdatesFromHostBehavior::Unknown => todo!(),
                                UpdatesFromHostBehavior::AllowOverwrite => {
                                    let result = db.update_data_into_partial_db(
                                        db_name,
                                        table_name,
                                        cmd,
                                        &known_host.host_id,
                                        &where_clause,
                                    );

                                    match result {
                                        Ok(data_result) => {
                                            if data_result.is_successful {
                                                let row = RowInfo {
                                                    database_name: db_name.to_string(),
                                                    table_name: table_name.to_string(),
                                                    row_id: data_result.row_id,
                                                    data_hash: data_result.data_hash.unwrap(),
                                                };
                                                rows.push(row);
                                                update_status = PartialDataStatus::to_u32(
                                                    PartialDataStatus::SucessOverwriteOrLog,
                                                );
                                                is_successful = true;
                                            }
                                        }
                                        Err(e) => error = Some(e.into()),
                                    }
                                }
                                UpdatesFromHostBehavior::QueueForReview => {
                                    let result = db.update_data_into_partial_db_queue(
                                        db_name,
                                        table_name,
                                        cmd,
                                        &where_clause,
                                        &known_host,
                                    );
                                    match result {
                                        Ok(data_result) => {
                                            if data_result.is_successful {
                                                update_status = PartialDataStatus::to_u32(
                                                    PartialDataStatus::Pending,
                                                );
                                                message =
                                                    String::from("The update statement has been logged for review");
                                                is_successful = true;
                                            }
                                        }
                                        Err(e) => error = Some(e.into()),
                                    }
                                }
                                UpdatesFromHostBehavior::OverwriteWithLog => {
                                    let result = db.update_data_into_partial_db(
                                        db_name,
                                        table_name,
                                        cmd,
                                        &known_host.host_id,
                                        &where_clause,
                                    );

                                    match result {
                                        Ok(data_result) => {
                                            if data_result.is_successful {
                                                let row = RowInfo {
                                                    database_name: db_name.to_string(),
                                                    table_name: table_name.to_string(),
                                                    row_id: data_result.row_id,
                                                    data_hash: data_result.data_hash.unwrap(),
                                                };
                                                rows.push(row);
                                                update_status = PartialDataStatus::to_u32(
                                                    PartialDataStatus::SucessOverwriteOrLog,
                                                );
                                                is_successful = true;
                                            }
                                        }
                                        Err(e) => error = Some(e.into()),
                                    }
                                }
                                UpdatesFromHostBehavior::Ignore => {
                                    message = format!(
                                        "The participant does not allow updates for db {db_name} table: {table_name}"
                                    );
                                    update_status =
                                        PartialDataStatus::to_u32(PartialDataStatus::Ignored);
                                    is_successful = false;
                                }
                                UpdatesFromHostBehavior::QueueForReviewAndLog => {
                                    let result = db.update_data_into_partial_db_queue(
                                        db_name,
                                        table_name,
                                        cmd,
                                        &where_clause,
                                        &known_host,
                                    );

                                    match result {
                                        Ok(data_result) => {
                                            if data_result.is_successful {
                                                update_status = PartialDataStatus::to_u32(
                                                    PartialDataStatus::Pending,
                                                );
                                                message =
                                                    String::from("The update statement has been logged for review");
                                                is_successful = true;
                                            }
                                        }
                                        Err(e) => error = Some(e.into()),
                                    }
                                }
                            }
                        }
                        Err(e) => error = Some(e.into()),
                    }
                }
            }
            Err(e) => error = Some(e.into()),
        }
    }

    UpdateDataResult {
        authentication_result: Some(auth.clone()),
        is_successful,
        message,
        rows,
        update_status,
        is_error: error.is_some(),
        error,
    }
}
