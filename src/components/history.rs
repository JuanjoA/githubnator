use leptos::prelude::*;

#[component]
pub fn History(get: ReadSignal<Vec<String>>) -> impl IntoView {
    view! {
        {move || {
            get.get()
                .into_iter()
                .rev()
                .map(|url| {
                    let url_clone = url.clone();
                    view! {
                        <div class="box">
                            <a class="has-text-primary-dark" href=url.clone() target="_blank">
                                {url_clone}
                            </a>
                        </div>
                    }
                })
                .collect::<Vec<_>>()
        }}
    }
}