use treaty_types::types::treaty_proto::PendingStatement;
use yew::{function_component, html, Callback, Html, Properties, UseStateHandle};

#[derive(Properties, PartialEq)]
pub struct PendingActionProps {
    pub pending_actions: UseStateHandle<Vec<PendingStatement>>,
    pub onclick_accept: Option<Callback<u32>>,
    pub onclick_reject: Option<Callback<u32>>,
}

#[function_component]
pub fn PendingActions(
    PendingActionProps {
        pending_actions,
        onclick_accept,
        onclick_reject,
    }: &PendingActionProps,
) -> Html {
    let pending_actions = pending_actions.clone();
    let onclick_accept = onclick_accept.clone();
    let onclick_reject = onclick_reject.clone();

    html!(
        <div>
            <div class="table-container">
            <table class="table is-narrow">
                <thead>
                    <tr>
                        <th>{"Row Id"}</th>
                        <th>{"Statement"}</th>
                        <th>{"Requested Timestamp UTC"}</th>
                        <th>{"From Host Id"}</th>
                        <th>{"Action"}</th>
                        <th>{"Accept/Reject?"}</th>
                    </tr>
                </thead>
                {
                    (*pending_actions).clone().into_iter().map(|a|{
                        let id = a.row_id;
                        let row_id = a.row_id.to_string();
                        let statement = a.statement.clone();
                        let requested_ts = a.requested_ts_utc.clone();
                        let host_id = a.host_id.clone();
                        let action = a.action.clone();

                        html!(
                            <tr>
                                <td>{row_id}</td>
                                <td>{statement}</td>
                                <td>{requested_ts}</td>
                                <td>{host_id}</td>
                                <td>{action}</td>
                                <td>
                                    <div class="buttons">
                                        <button type="button" class="button is-success" id="accept_statement" value="Accept Statement"
                                        onclick=
                                        {
                                            let onclick_accept = onclick_accept.clone();
                                            Callback::from(move |_| {
                                                let onclick_accept = onclick_accept.clone();
                                                if let Some(x) = onclick_accept {
                                                    x.emit(id);
                                                }
                                            }
                                        )}>
                                        <span class="mdi mdi-file-document-check">{" Accept"}</span>
                                        </button>

                                        <button type="button" class="button is-danger" id="reject_statement" value="Reject Statement"
                                        onclick=
                                        {
                                            let onclick_reject = onclick_reject.clone();
                                            Callback::from(move |_| {
                                                let onclick_reject = onclick_reject.clone();

                                                if let Some(x) = onclick_reject {
                                                    x.emit(id);
                                                }
                                            }
                                        )}
                                        >
                                        <span class="mdi mdi-file-document-remove">{" Reject"}</span>
                                        </button>
                                    </div>
                                </td>
                            </tr>
                        )

                    }).collect::<Html>()
                }
            </table>
            </div>
        </div>
    )
}
