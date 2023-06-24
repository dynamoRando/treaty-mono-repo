use yew::prelude::*;

use crate::request::treaty::get_treaty_token;

#[derive(Properties, PartialEq, Debug)]
pub struct StatusProps {
    pub is_logged_in: UseStateHandle<bool>,
    pub status_message: UseStateHandle<String>,
}

#[function_component]
pub fn Status(props: &StatusProps) -> Html {
    let is_logged_in_state = props.is_logged_in.clone();
    let text = &*props.status_message.clone();
    let addr = get_treaty_token().addr;

    html!(
        <div>
            <nav class="navbar is-link has-shadow">
                <div class="navbar-brand">
                    <h3 class="navbar-item is-size-4">{"Status"}</h3>
                    <div class="navbar-item">
                        <div class="buttons">
                        {
                            if *is_logged_in_state {
                                html! {
                                    <div>
                                    <button class="button is-success">
                                        <span class="mdi mdi-account-check">{" Logged In"}</span>
                                    </button>
                                    <button class="button is-warning">{ addr }
                                    </button>
                                    </div>
                                }
                            }
                            else {
                                html! {
                                    <button class="button is-light">
                                    <span class="mdi mdi-account-cancel">{" Not Logged In"}</span>
                                    </button>
                                    }
                            }
                        }
                        </div>
                    </div>
                    <div class="navbar-item">
                        <div class="field">
                        <input type="text" class="input" size=95
                        id ="status_message" placeholder="treaty Status Messages Will Appear Here"
                        value={text.clone()} readonly=true />
                        </div>
                    </div>

                </div>
            </nav>
        </div>
    )
}
