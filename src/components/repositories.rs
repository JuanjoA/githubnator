use leptos::*;
use web_sys::MouseEvent;
use uuid::Uuid;
use crate::models::{GlobalState, RepositoryButton, get_default_repositories, get_default_url_repository, get_default_url_model};
use gloo_storage::Storage;
use logging::log;
use html::Button;



#[component]
pub fn Repositories<F>(
    clear_action: RwSignal<bool>,
    repo_get: ReadSignal<String>,
    on_click: F,
) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    let state = expect_context::<RwSignal<GlobalState>>();
    let storage = gloo_storage::LocalStorage::raw();
    let data_localstorage = storage.get("repositories");
    let textarea_data_from_cache: String = match data_localstorage {
        Ok(value) => match value {
            Some(val) => val,
            None => get_default_repositories(),
        },
        Err(e) => {
            log!("> Error: {:?}", e);
            String::new()
        }
    };

    let (textarea_data, textarea_data_set) = create_signal(textarea_data_from_cache.clone());

    //
    // Making the buttons
    //
    let buttons = move || {
        textarea_data()
            .trim()
            .split("\n")
            .into_iter()
            .map(|repo_name| repo_name.to_string())
            .filter(|repo_name| repo_name != "")
            .map(|repo_name| {
                state.update(|state| {
                    state.buttons.insert(repo_name.clone(), false);
                });
                create_rw_signal(RepositoryButton::new(Uuid::new_v4(), repo_name, false))
            })
            .collect::<Vec<_>>()
    };
    create_effect(move |_| {
        let _value = clear_action.get();
        let actual_textarea = textarea_data();
        textarea_data_set(actual_textarea);
        
    });
    let (is_hidden, is_hidden_set) = create_signal(true);

    view! {
        <div class="columns is-centered has-text-centered px-2 py-2">
            <div class="">
                //
                // Action config
                //
                <button
                    class="button is-primary is-outlined mr-1"
                    on:click=move |_| {
                        is_hidden_set(!is_hidden());
                    }
                    aria-label="Clear all filters and repositories"
                    name="clear-button"
                >
                    <span class="icon">
                        <i class="fas fa-gear"></i>
                    </span>
                </button>
                //
                // Action clear filters
                //
                <button
                    class="button is-primary is-outlined"
                    disabled=move || {
                        repo_get().contains(get_default_url_repository().as_str())
                            & state().model.contains(get_default_url_model().as_str())
                    }
                    on:click=on_click
                    aria-label="Open configuration settings"
                    name="config-button"
                >
                    <span class="icon">
                        <i class="fa-solid fa-circle-xmark"></i>
                    </span>
                </button>
            </div>
        </div>
        //
        // Textarea-input repositories
        //
        <div class="mx-2">
            <div class="field">
            <textarea
            id="repo_text"
            name="repositories"
            rows="7"
            cols="33"
            placeholder="One repo by line, only path (exclude domain). Unfocus to update."
            class="textarea is-primary is-small"
            class:is-hidden=move || is_hidden()
            on:change=move |ev| {
                textarea_data_set(event_target_value(&ev));
                let _ = storage.set("repositories", &event_target_value(&ev));
            }
            on:blur=move |_| {
                is_hidden_set(true);
            }
        >
            {textarea_data}
        </textarea>      
            </div>
        </div>
        
        //
        // REPO BUTTONS
        //
        <div class="columns is-centered buttons mt-2 px-2 py-2">
            {move || {
                buttons()
                    .into_iter()
                    .map(|rb| {
                        let noderef: NodeRef<Button> = create_node_ref();
                        view! {
                            <div class="column is-3 has-text-centered">
                                <button node_ref=noderef
                                    id={move || rb.get().id.to_string()}
                                    class="button is-primary is-outlined"
                                    class:is-focused=move || {
                                        rb.get().active
                                    }
                                    on:click=move |_| {
                                        let new_value = !rb.get().active;
                                        state.update(|state| {
                                            state.message = format!("> Button clicked! ({:?})", rb.get().label);
                                            // hasmap cant repeat keys. if exists, this replace the value:
                                            state.buttons.insert(rb.get().label, new_value);
                                        });
                                        rb.update(|b| {
                                            b.active = new_value;
                                        });
                                        if new_value == false {
                                            let node = noderef.get().expect("node_ref should be loaded by now");
                                            let _ = node.blur();
                                        };
                                    }
                                    on:focusout=move |_| {
                                    }
                                >{move || format!("{}", rb.get().label)}
                                </button>
                            </div>
                        }
                    })
                    .collect::<Vec<_>>()
            }}
        </div>
    }
}