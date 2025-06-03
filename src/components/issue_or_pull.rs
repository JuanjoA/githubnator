use leptos::*;
use leptos::prelude::*;
#[component]
pub fn IssueOrPull() -> impl IntoView {
    let state = expect_context::<RwSignal<crate::GlobalState>>();
    
    let (issueorpull, issueorpull_set) = create_slice(
        state,
        |state| state.model.clone(),
        |state, value| state.model = value,
    );

    view! {
        <div class="columns is-centered is-mobile">
            <div class="column is-narrow has-text-centered">
                <button
                    class="button is-primary is-outlined"
                    class:is-active=move || issueorpull.get() == "issues".to_string()
                    class:is-focused=move || issueorpull.get() == "issues".to_string()
                    on:click=move |_| issueorpull_set.set("issues".to_string())
                >
                    issues
                </button>
            </div>
            <div class="column is-narrow has-text-centered">
                <button
                    class="button is-primary is-outlined"
                    class:is-active=move || issueorpull.get() == "pulls".to_string()
                    class:is-focused=move || issueorpull.get() == "pulls".to_string()
                    on:click=move |_| issueorpull_set.set("pulls".to_string())
                >
                    pulls
                </button>
            </div>
        </div>
    }
}