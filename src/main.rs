mod components;
use components::{HelpIcon, InputQueryBy, IssueOrPull, Repositories, History};


use leptos::*;
use leptos::prelude::*;
use leptos::mount::mount_to_body;


mod models;
use crate::models::{GlobalState, get_default_url_repository, get_default_url_model, get_default_url_id};

fn main() {
    mount_to_body(App);
}

fn open_in_new_tab(url: &str) {
    // NOTE: El usuario debe autorizar en el navegador el abrir múltiples tabs
    // o solo saldrá la primera de ellas. Mirar barra de url en navegador.
    let _result = web_sys::window()
        .unwrap()
        .open_with_url_and_target(url, "_blank")
        .expect("Fail to open Window!");
}

#[component]
fn App() -> impl IntoView {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    let state = RwSignal::new(GlobalState::default());
    provide_context(state);

    let repository_name = signal(get_default_url_repository());
    let (_id_get, _id_set) = signal(get_default_url_id());

    let (_button_now, _button_now_set) = signal("".to_string());

    let (query_by_string, query_by_string_set) = signal("".to_string());
    let (query_by_author, query_by_author_set) = signal("".to_string());
    let (query_by_label, query_by_label_set) = signal("".to_string());
    let (query_by_assignee, query_by_assignee_set) = signal("".to_string());
    let (query_by_state, query_by_state_set) = signal("".to_string());
    let (query_by_in, query_by_in_set) = signal("".to_string());

    let url_to_go = signal("".to_string());

    let (url_history, url_history_set) = signal(Vec::<String>::new());

    let clear_action = RwSignal::new(false);

    let clean_filters = move || {
        query_by_string_set.set("".to_string());
        query_by_author_set.set("".to_string());
        query_by_label_set.set("".to_string());
        query_by_assignee_set.set("".to_string());
        query_by_state_set.set("".to_string());
        query_by_in_set.set("".to_string());
        repository_name.1.set(get_default_url_repository());
        state.update(|state| {
            state.model = get_default_url_model();
        });
        clear_action.set(!clear_action.get());
    };

    let query_part = move || {
        let mut query = "".to_string();
        let mut first_arg = true;
        
        if !query_by_state.get().is_empty() {
            match query_by_state.get().as_str() {
                "open" => query += " is:open",
                "closed" => query += " is:closed",
                "merged" => query += " is:merged",
                "draft" => query += " draft:true",
                _ => {}
            }
            first_arg = false;
        }
        
        if !query_by_in.get().is_empty() {
            if !first_arg {
                query += " ";
            }
            match query_by_in.get().as_str() {
                "title" => query += "in:title",
                "body" => query += "in:body",
                "comments" => query += "in:comments",
                _ => {}
            }
            first_arg = false;
        }
        
        if state.get().model == "issues" {
            if !first_arg {
                query += " ";
            }
            first_arg = false;
            query += "is:issue";
        }
        if query_by_author.get() != "".to_string() {
            if !first_arg {
                query += " ";
            }
            first_arg = false;
            query += format!("author:{}", query_by_author.get()).as_str();
        }
        if query_by_label.get() != "".to_string() {
            let query_by_label_result = query_by_label.get().clone();

            let labels: Vec<&str> = query_by_label_result
                .split(|c| c == ',' || c == ' ')
                .filter(|s| !s.is_empty())
                .collect();

            if first_arg == false {
                query += " ";
            }
            first_arg = false;
            labels.iter().for_each(|label| {
                query += format!("label:{} ", label).as_str();
            });
        }
        if query_by_assignee.get() != "".to_string() {
            if !first_arg {
                query += " ";
            }
            first_arg = false;
            query += format!("assignee:{}", query_by_assignee.get()).as_str();
        }
        if query_by_string.get() != "".to_string() {
            if !first_arg {
                query += " ";
            }
            query += format!("{}", query_by_string.get()).as_str();
        }
        query
    };

    let url_target = move || {
        let binding = state.get();
        let selected_repos: Vec<&String> = binding
            .buttons
            .iter()
            .filter(|(_, is_active)| **is_active)
            .map(|(repo_name, _)| repo_name)
            .collect();
        
        let repo_part = match selected_repos.len() {
            0 => get_default_url_repository(),
            1 => selected_repos[0].clone(),
            _ => "<multiple-repos-selected>".to_string(),
        };
        
        let has_filters = query_by_string.get() != "" 
            || query_by_author.get() != "" 
            || query_by_label.get() != ""
            || query_by_assignee.get() != ""
            || query_by_state.get() != ""
            || query_by_in.get() != "";
        
        let mut new_url = format!(
            "https://github.com/{}/{}",
            repo_part,
            state.get().model
        );
        
        if has_filters {
            let query_args = query_part();
            if !query_args.trim().is_empty() {
                new_url.push_str(&format!("?q={}", query_args));
            }
        }
        
        url_to_go.1.set(new_url.clone());
        new_url
    };

    let submit_disabled = move || {
        let binding = state.get();
        let we_have_repo = binding
            .buttons
            .iter()
            .filter(|(_k, v)| **v == true)
            .map(|(_k, v)| v)
            .collect::<Vec<_>>();
        if state.get().model != "" && we_have_repo.len() > 0 {
            false
        } else {true}
        
    };

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let args: String = format!("?q={}", query_part());
        
        let mut urls: Vec<String> = url_history.get();
        let mut new_urls = Vec::new();

        state.get().buttons.iter()
            .filter(|(_, active)| **active)
            .for_each(|(key_label, _)| {
                let url = format!("https://github.com/{}/{}/{}", key_label, state.get().model, args);
                open_in_new_tab(&url);
                new_urls.push(url);
            });

        if !new_urls.is_empty() {
            urls.extend(new_urls);
            if urls.len() > 10 {
                urls = urls[urls.len() - 10..].to_vec();
            }
            url_history_set.set(urls);
        }
    };

    view! {
        <div class="header-section mt-4">
            <div class="columns is-vcentered">
                <div class="column is-narrow is-hidden-mobile"></div>
                <div class="column has-text-centered">
                    <h1 class="title has-text-primary-dark">
                        "GitHub Linker"
                    </h1>
                </div>
                <div class="column is-narrow has-text-right mr-2">
                    <HelpIcon/>
                </div>
            </div>
        </div>
        <p class="subtitle has-text-primary-dark has-text-centered">{url_target}</p>
        <IssueOrPull />
        <Repositories
            clear_action=clear_action
            repo_get=repository_name.0  // signal = (RwSignal, RwSignalSet)
            on_click=move |_| {
                clean_filters();
            }
        />

        <div class="mb-3"></div>

        <form method="GET" action=url_target target="_blank" on:submit=on_submit>
            // https://stackoverflow.com/questions/1116019/when-submitting-a-get-form-the-query-string-is-removed-from-the-action-url
            // the query params source is the fields on form, cant force in action.
            // <input type="search" name="q" value={query_part()}/>
            <input type="hidden" name="q" prop:value=move || query_part() />
            <div class="columns is-multiline is-mobile">
                <div class="column is-half-mobile is-one-quarter-desktop">
                    <InputQueryBy
                        value_get=query_by_string
                        value_set=query_by_string_set
                        placeholder="Filter by ...".to_string()
                        class="fas fa-magnifying-glass".to_string()
                    />
                </div>
                <div class="column is-half-mobile is-one-quarter-desktop">
                    <div class="field has-text-centered px-2 py-2">
                        <div class="control has-icons-left">
                            <div class="select is-fullwidth">
                                <select
                                    class="input is-primary"
                                    style="height: auto; padding-right: 2.5em;"
                                    on:change=move |ev| {
                                        let value = event_target_value(&ev);
                                        query_by_in_set.set(value);
                                    }
                                    prop:value=move || query_by_in.get()
                                    class:has-text-grey-light=move || query_by_in.get().is_empty()
                                >
                                    <option value="" class="has-text-grey-light">"Search in ..."</option>
                                    <option value="title">"Title"</option>
                                    <option value="body">"Body"</option>
                                    <option value="comments">"Comments"</option>
                                </select>
                            </div>
                            <span class="icon is-left">
                                <i class="fas fa-location-dot"></i>
                            </span>
                        </div>
                    </div>
                </div>
                <div class="column is-half-mobile is-one-quarter-desktop">
                    <InputQueryBy
                        value_get=query_by_author
                        value_set=query_by_author_set
                        placeholder="Filter by author".to_string()
                        class="fas fa-user".to_string()
                    />
                </div>
                <div class="column is-half-mobile is-one-quarter-desktop">
                    <InputQueryBy
                        value_get=query_by_label
                        value_set=query_by_label_set
                        placeholder="Filter by label".to_string()
                        class="fas fa-tag".to_string()
                    />
                </div>
                <div class="column is-half-mobile is-one-quarter-desktop">
                    <InputQueryBy
                        value_get=query_by_assignee
                        value_set=query_by_assignee_set
                        placeholder="Filter by assignee".to_string()
                        class="fas fa-person-digging".to_string()
                    />
                </div>
                <div class="column is-half-mobile is-one-quarter-desktop">
                    <div class="field has-text-centered px-2 py-2">
                        <div class="control has-icons-left">
                            <div class="select is-fullwidth">
                                <select
                                    class="input is-primary"
                                    style="height: auto; padding-right: 2.5em;"
                                    on:change=move |ev| {
                                        let value = event_target_value(&ev);
                                        query_by_state_set.set(value);
                                    }
                                    prop:value=move || query_by_state.get()
                                    class:has-text-grey-light=move || query_by_state.get().is_empty()
                                >
                                    <option value="" class="has-text-grey-light">"Filter by state ..."</option>
                                    <option value="open">"Open"</option>
                                    <option value="closed">"Closed"</option>
                                    {move || {
                                        if state.get().model == "pulls" {
                                            Some(view! {
                                                <>
                                                    <option value="merged">"Merged"</option>
                                                    <option value="draft">"Draft"</option>
                                                </>
                                            })
                                        } else {
                                            None
                                        }
                                    }}
                                </select>
                            </div>
                            <span class="icon is-left">
                                <i class="fas fa-circle-check"></i>
                            </span>
                        </div>
                    </div>
                </div>
            </div>
            <div class="columns is-centered is-mobile">
                <div class="column is-narrow">
                    <button
                        class="button is-primary is-light is-outlined mx-1 my-1"
                        disabled=submit_disabled
                        type="submit"
                    >
                        GO
                    </button>
                </div>
            </div>

            <History get=url_history />
        </form>
    }
}

#[cfg(test)]
mod tests {

    fn get_vec_from_text(data: String, separator: &str) -> Vec<String> {
        data.split(separator)
            .collect::<Vec<&str>>()
            .iter()
            .map(|val| val.to_string())
            .collect::<Vec<String>>()
    }

    #[test]
    fn get_vec_from_text_test() {
        let data: String = "odoo/odoo\nOCA/OCB".to_string();
        let separator = "\n";
        let result_vec = get_vec_from_text(data, separator);
        assert_eq!(2, result_vec.len());
    }
}
