use leptos::{html::Button, *};
use leptos::prelude::*;
use web_sys::MouseEvent;
use uuid::Uuid;
use crate::models::{GlobalState, RepositoryButton, get_default_repositories, get_default_url_repository, get_default_url_model};
use gloo_storage::Storage;
use log::error;
use leptos_dom::helpers::event_target_value;



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
    let initial_data: String = match data_localstorage {
        Ok(value) => match value {
            Some(val) => val,
            None => get_default_repositories(),
        },
        Err(e) => {
            error!("> Error: {:?}", e);
            get_default_repositories()
        }
    };

    let (is_hidden, is_hidden_set) = signal(true);
    let (textarea_data, textarea_data_set) = signal(initial_data);

        let buttons = Memo::new(move |_| {
        textarea_data.get()
            .trim()
            .split("\n")
            .into_iter()
            .map(|repo_name| repo_name.to_string())
            .filter(|repo_name| !repo_name.is_empty())
            .map(|repo_name| {
                let is_active = state.get().buttons.get(&repo_name).copied().unwrap_or(false);
                RwSignal::new(RepositoryButton::new(Uuid::new_v4(), repo_name, is_active))
            })
            .collect::<Vec<_>>()
    });

    Effect::new(move |_| {
        let _value = clear_action.get();
        
        state.update(|state| {
            // Desactivar todos los botones sin eliminarlos
            for (_, active) in state.buttons.iter_mut() {
                *active = false;
            }
        });
    });

    view! {
        <div class="columns is-centered has-text-centered px-2 py-2">
            <div class="">
                //
                // Action config
                //
                <button
                    class="button is-primary is-outlined mr-1"
                    on:click=move |_| {
                        is_hidden_set.set(!is_hidden.get());
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
                        repo_get.get().contains(get_default_url_repository().as_str())
                            & state.get().model.contains(get_default_url_model().as_str())
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
                class:is-hidden=move || is_hidden.get()
                on:input=move |ev| {
                    let value = event_target_value(&ev);
                    textarea_data_set.set(value.clone());
                }
                on:change=move |ev| {
                    let value = event_target_value(&ev);
                    let _ = storage.set("repositories", &value);
                    state.update(|state| {
                        state.buttons.clear();
                    });
                }
                on:blur=move |_| {
                    is_hidden_set.set(true);
                }
                prop:value=move || textarea_data.get()
            />
            </div>
        </div>
        
        //
        // REPO BUTTONS
        //
        <div class="columns is-centered buttons mt-2 px-2 py-2">
            {move || {
                buttons.get()
                    .into_iter()
                    .map(|rb| {
                        let noderef: NodeRef<Button> = NodeRef::new();
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