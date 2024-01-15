use stdext::function_name;
use tracing::{trace, warn};

use crate::{
    db_interface::dbi_actions::DbiActions,
    error::TreatyDbError,
    remote::remote_actions::RemoteActions,
    treaty_proto::{ExecuteReadRequest, Row, TreatyWarning},
};

#[derive(Debug, Clone)]
pub struct ReadResult {
    pub rows_affected: u32,
    pub rows: Vec<Row>,
    pub warning: Option<TreatyWarning>,
}

pub async fn handle_execute_read_at_host<T: DbiActions + Clone, R: RemoteActions + Clone>(
    db: &T,
    remote: &R,
    request: &ExecuteReadRequest,
) -> Result<ReadResult, TreatyDbError> {
    let db_name = &request.database_name;
    let sql = &request.sql_statement;
    let mut read_result = ReadResult {
        rows_affected: 0,
        rows: Vec::new(),
        warning: None,
    };

    let has_cooperative_tables = db.has_cooperative_tables(db_name, sql).await?;

    if has_cooperative_tables {
        trace!(
            "[{}]: found cooperative tables for: {} w/ sql {}",
            function_name!(),
            &db_name,
            &sql
        );

        let cooperative_tables = db
            .get_cooperative_tables(db_name, sql)
            .await?
            .unwrap_or_default();

        trace!(
            "[{}]: cooperative_tables: {cooperative_tables:?}",
            function_name!()
        );

        for ct in &cooperative_tables {
            let participants_for_table = db
                .get_participants_for_table(db_name, ct.as_str())
                .await?
                .unwrap_or_default();

            trace!(
                "[{}]: execute_read_at_host: participants_for_table: {participants_for_table:?}",
                function_name!()
            );

            if participants_for_table.is_empty() {
                warn!(
                    "[{}]: execute_read_at_host: no participants found for table: {ct:?}",
                    function_name!()
                );
            }

            for participant in &participants_for_table {
                trace!("execute_read_at_host: participant: {participant:?}");

                for row in &participant.row_data {
                    trace!("[{}]: row: {row:?}", function_name!());

                    // we would need to get rows for that table from the participant
                    let host_info = db
                        .treaty_get_host_info()
                        .await
                        .expect("no host info is set")
                        .unwrap_or_default();

                    let remote_data_result = remote
                        .get_row_from_participant(participant.clone(), host_info, row.0)
                        .await;

                    trace!(
                        "[{}]: execute_read_at_host: remote_data_result: {remote_data_result:?}",
                        function_name!()
                    );

                    if !remote_data_result.is_successful {
                        warn!(
                            "[{}]: remote data result failed: {remote_data_result:?}",
                            function_name!()
                        );
                    }

                    let data_hash_for_row = remote_data_result.row.as_ref().unwrap().hash.clone();
                    trace!(
                        "[{}]: data_hash_for_row: {data_hash_for_row:?}",
                        function_name!()
                    );

                    let saved_hash_for_row = row.1.clone();
                    trace!(
                        "[{}]: saved_hash_for_row: {saved_hash_for_row:?}",
                        function_name!()
                    );

                    if data_hash_for_row == saved_hash_for_row {
                        let row = remote_data_result.row.as_ref().unwrap().clone();
                        read_result.rows.push(row);
                    } else {
                        let row = remote_data_result.row.as_ref().unwrap().clone();
                        read_result.rows.push(row);

                        warn!(
                            "[{}]: data hashes for host and participant rows do not match!",
                            function_name!()
                        );

                        let warning = TreatyWarning {
                            message: String::from(
                                "warning: data hashes for host and participant rows do not match!",
                            ),
                            help: Some(String::from("This could be intentional if the behaviors configured between host and participant do not sync changes")),
                            number: 0,
                        };

                        read_result.warning = Some(warning);
                    }
                }
            }
        }

        if sql.contains("WHERE") {
            warn!("[{}]: tables with cooperative data will NOT be filtered, statement has a filter: {sql:?}", function_name!());
            warn!(
                "[{}]: this functionality has yet to be implemented",
                function_name!()
            );
        }
    } else {
        let query_result = db.execute_read_at_host(db_name, sql).await?;
        let result_rows = query_result.convert_to_protobuf();
        read_result.rows_affected = result_rows.len() as u32;
        read_result.rows = result_rows;
    }

    Ok(read_result)
}
