use leptos::*;

#[component]
pub fn History(get: ReadSignal<Vec<String>>) -> impl IntoView {
    view! {
        {move || {
            get()
                .into_iter()
                .rev()
                .map(|url| {
                    view! {
                        <div class="box">
                            <a class="has-text-primary-dark" href=&url target="_blank">
                                {url}
                            </a>
                        </div>
                    }
                })
                .collect::<Vec<_>>()
        }}
    }
}