use stdext::function_name;
use tracing::trace;

use crate::{
    db_interface::dbi_actions::DbiActions,
    treaty_proto::{AuthResult, DeleteDataRequest, DeleteDataResult, RowInfo, TreatyError},
};

use treaty_types::enums::{DeletesFromHostBehavior};


pub async fn handle_delete_command_into_table<T: DbiActions + Clone>(
    db: &T,
    request: &DeleteDataRequest,
    auth: &AuthResult,
) -> DeleteDataResult {
    let db_name = &request.database_name;
    let table_name = &request.table_name;
    let where_clause = &request.where_clause.clone();
    let cmd = &request.cmd;

    trace!("[{}] {cmd:?}", function_name!());

    let mut error: Option<TreatyError> = None;
    let mut message = String::from("");
    let mut rows: Vec<RowInfo> = Vec::new();
    let mut is_successful = false;

    if auth.is_authenticated {
        let result = db.get_cds_host_for_part_db(db_name);
        match result {
            Ok(opt_known_host) => {
                if let Some(known_host) = opt_known_host {
                    let result = db.get_deletes_from_host_behavior(db_name, table_name);
                    match result {
                        Ok(behavior) => match behavior {
                            DeletesFromHostBehavior::Unknown => todo!(),
                            DeletesFromHostBehavior::AllowRemoval => {
                                let result = db.delete_data_in_partial_db(
                                    db_name,
                                    table_name,
                                    cmd,
                                    where_clause,
                                    &known_host.host_id,
                                );

                                trace!("[{}] {result:?}", function_name!());

                                match result {
                                    Ok(data_result) => {
                                        if data_result.is_successful {
                                            is_successful = true;

                                            let hash = match data_result.data_hash {
                                                Some(_) => data_result.data_hash.unwrap(),
                                                None => 0,
                                            };

                                            let row = RowInfo {
                                                database_name: db_name.to_string(),
                                                table_name: table_name.to_string(),
                                                row_id: data_result.row_id,
                                                data_hash: hash,
                                            };
                                            rows.push(row);
                                        }
                                    }
                                    Err(e) => error = Some(e.into()),
                                }
                            }
                            DeletesFromHostBehavior::QueueForReview => {
                                let result = db.delete_data_in_partial_db(
                                    db_name,
                                    table_name,
                                    cmd,
                                    where_clause,
                                    &known_host.host_id,
                                );

                                match result {
                                    Ok(data_result) => {
                                        if data_result.is_successful {
                                            message = String::from(
                                                "The delete statement has been logged for review",
                                            );
                                        }
                                    }
                                    Err(e) => error = Some(e.into()),
                                }
                            }
                            DeletesFromHostBehavior::DeleteWithLog => {
                                let result = db.delete_data_in_partial_db(
                                    db_name,
                                    table_name,
                                    cmd,
                                    where_clause,
                                    &known_host.host_id,
                                );

                                match result {
                                    Ok(data_result) => {
                                        if data_result.is_successful {
                                            is_successful = true;

                                            let hash = match data_result.data_hash {
                                                Some(_) => data_result.data_hash.unwrap(),
                                                None => 0,
                                            };

                                            if data_result.is_successful {
                                                let row = RowInfo {
                                                    database_name: db_name.to_string(),
                                                    table_name: table_name.to_string(),
                                                    row_id: data_result.row_id,
                                                    data_hash: hash,
                                                };
                                                rows.push(row);
                                            }
                                        }
                                    }
                                    Err(e) => error = Some(e.into()),
                                }
                            }
                            DeletesFromHostBehavior::Ignore => {
                                message = format!(
                                        "The participant does not allow updates for db {db_name} table: {table_name}"
                                    );
                                is_successful = true;
                            }
                            DeletesFromHostBehavior::QueueForReviewAndLog => todo!(),
                        },
                        Err(e) => error = Some(e.into()),
                    }
                }
            }
            Err(e) => error = Some(e.into()),
        }
    }

    DeleteDataResult {
        authentication_result: Some(auth.clone()),
        is_successful,
        message,
        rows,
        is_error: error.is_some(),
        error,
    }
}
