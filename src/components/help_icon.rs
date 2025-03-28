use leptos::*;


#[component]
pub fn HelpIcon() -> impl IntoView {
    let (show_tooltip, set_show_tooltip) = create_signal(false);
    
    view! {
        <div class="help-icon-container" style="position: relative;">
            <span 
                class="icon has-text-primary-dark is-clickable" 
                on:click=move |_| set_show_tooltip.update(|v| *v = !*v)
                style="margin-left: 8px; font-size: 1.2rem;"
            >
                <i class="fas fa-question-circle"></i>
            </span>
            
            {move || show_tooltip().then(|| view! {
                <div 
                    class="notification is-primary-light has-text-dark notification is-info is-light" 
                    style="position: absolute; right: 0; top: 100%; z-index: 100; width: 280px; padding: 1rem; font-size: 0.9rem; text-align: left; border-radius: 4px; box-shadow: 0 2px 5px rgba(0,0,0,0.2);"
                >
                    
                    <div class="content mt-2">
                        <h4 class="has-text-centered"><i class="fas fa-shield-alt mr-2"></i>Privacy Guaranteed</h4>
                        <p>
                            This application fully respects your privacy:
                        </p>
                        <ul>
                            <li><strong>No data storage: </strong>{"We don't save any of your filters, queries, or preferences on our servers."}</li>
                            <li><strong>Everything stays local: </strong>{"The entire application runs in your browser. We don't send information to external servers."}</li>
                            <li><strong>No tracking: </strong>{"We don't use tracking cookies or analyze your behavior."}</li>
                        </ul>
                        <p class="has-text-centered is-italic">
                            Your GitHub searches remain private and under your control at all times.
                        </p>
                    </div>

                    <div class="content mt-2">
                        <h4 class="has-text-centered"><i class="fas fa-exclamation-triangle mr-2"></i>Multiple Tabs Notice</h4>
                        
                        <p>
                            You must allow your browser to open multiple tabs at once when using this application for the first time.
                        </p>
                    </div>
                    
                    <button 
                        class="delete is-small" 
                        style="position: absolute; top: 0.5rem; right: 0.5rem;"
                        on:click=move |e| {
                            e.stop_propagation();
                            set_show_tooltip(false);
                        }
                    ></button>
                </div>
            })}
        </div>
    }
}