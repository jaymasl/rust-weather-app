use yew::prelude::*;
use crate::styles::Styles;

#[derive(Properties, PartialEq)]
pub struct SearchFormProps {
    pub onsubmit: Callback<SubmitEvent>,
    pub oninput: Callback<InputEvent>,
    pub toggle_theme: Callback<MouseEvent>,
    pub search_input: String,
    pub is_dark: bool,
    pub loading: bool,
}

#[function_component(SearchForm)]
pub fn search_form(props: &SearchFormProps) -> Html {
    html! {
        <div class={classes!(Styles::CARD_BASE, "max-w-sm", "mx-auto")}>
            <h1 class={Styles::HEADING_1}>{"Current Weather"}</h1>
            <a 
                href="https://www.jaykrown.com" 
                class={classes!("text-blue-500", "hover:text-blue-700", "text-center", "block", "mb-2")}
                target="_blank"
            >
                {"jaykrown.com"}
            </a>
            <form onsubmit={props.onsubmit.clone()}>
                <div class={classes!("flex", "flex-col", "space-y-2", "max-w-[280px]", "mx-auto")}>
                    <div class="flex items-center gap-2">
                        <input
                            type="text"
                            placeholder="Search for a city..."
                            value={props.search_input.clone()}
                            oninput={props.oninput.clone()}
                            class={classes!(Styles::SEARCH_INPUT, "flex-1")}
                        />
                        <button 
                            onclick={props.toggle_theme.clone()} 
                            class={Styles::THEME_TOGGLE} 
                            aria-label="Toggle dark mode"
                            type="button"
                        >
                            if props.is_dark {
                                <svg class={classes!(Styles::THEME_ICON, "text-orange-500")} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
                                </svg>
                            } else {
                                <svg class={classes!(Styles::THEME_ICON, "text-stone-900")} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
                                </svg>
                            }
                        </button>
                    </div>
                    <button
                        type="submit"
                        disabled={props.loading}
                        class={Styles::SEARCH_BUTTON}
                    >
                        {"Search"}
                    </button>
                </div>
            </form>
        </div>
    }
}