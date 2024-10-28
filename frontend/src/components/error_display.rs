use yew::prelude::*;
use crate::styles::Styles;

#[derive(Properties, PartialEq)]
pub struct ErrorProps {
    pub error: Option<String>,
    pub rate_limited: bool,
}

#[function_component(ErrorDisplay)]
pub fn error_display(props: &ErrorProps) -> Html {
    if let Some(err) = props.error.clone() {
        html! {
            <div class={classes!(
                Styles::CARD_BASE,
                if props.rate_limited {
                    classes!(
                        "bg-yellow-50",
                        "dark:bg-yellow-900/20",
                        "border-yellow-200",
                        "dark:border-yellow-800"
                    )
                } else {
                    classes!(
                        "bg-red-50",
                        "dark:bg-red-900/20",
                        "border-red-200",
                        "dark:border-red-800"
                    )
                }
            )}>
                <div class="flex items-center gap-2">
                    if props.rate_limited {
                        <svg class="w-5 h-5 text-yellow-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                                d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
                        </svg>
                    } else {
                        <svg class="w-5 h-5 text-red-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                                d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                        </svg>
                    }
                    <p class={classes!(
                        if props.rate_limited {
                            "text-yellow-800 dark:text-yellow-200"
                        } else {
                            "text-red-800 dark:text-red-200"
                        }
                    )}>
                        {err}
                    </p>
                </div>
            </div>
        }
    } else {
        html! {}
    }
}