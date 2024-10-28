mod styles;
mod components;
mod utils;

use components::*;
use crate::components::ErrorDisplay;
use crate::styles::Styles;
use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use weather_app_shared::WeatherData;
use web_sys::HtmlInputElement;

#[function_component(App)]
fn app() -> Html {
    let weather_data = use_state(|| None::<WeatherData>);
    let loading = use_state(|| false);
    let error = use_state(|| None::<String>);
    let search_input = use_state(|| String::new());
    let is_dark = use_state(|| utils::is_dark_mode());
    let rate_limited = use_state(|| false);

    let onsubmit = {
        let weather_data = weather_data.clone();
        let error = error.clone();
        let loading = loading.clone();
        let search_input = search_input.clone();
        let rate_limited = rate_limited.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let city = (*search_input).clone();
            if city.is_empty() {
                error.set(Some("Please enter a city name".to_string()));
                return;
            }
            
            error.set(None);
            rate_limited.set(false);
            loading.set(true);
            
            let weather_data = weather_data.clone();
            let error = error.clone();
            let loading = loading.clone();
            let rate_limited = rate_limited.clone();
            
            spawn_local(async move {
                let api_base = "/api";
                let url = format!("{}/weather/{}", api_base, city);
                println!("Sending request to: {}", url);
            
                match Request::get(&url)
                    .header("Accept", "application/json")
                    .header("Content-Type", "application/json")
                    .credentials(web_sys::RequestCredentials::Include)
                    .send()
                    .await 
                {
                    Ok(response) => {
                        println!("Response status: {}", response.status());
                        let headers = response.headers();
                        println!("Response headers: {:?}", headers);
                        
                        match response.status() {
                            200 => {
                                let text = response.text().await;
                                println!("Response text: {:?}", text);
                                
                                match text {
                                    Ok(text) => {
                                        match serde_json::from_str::<WeatherData>(&text) {
                                            Ok(data) => {
                                                println!("Successfully parsed weather data");
                                                weather_data.set(Some(data));
                                            },
                                            Err(e) => {
                                                println!("Error parsing JSON: {:?}", e);
                                                error.set(Some(
                                                    "Failed to parse weather data. Please try again.".to_string()
                                                ));
                                            }
                                        }
                                    },
                                    Err(e) => {
                                        println!("Error getting response text: {:?}", e);
                                        error.set(Some(
                                            "Failed to read response. Please try again.".to_string()
                                        ));
                                    }
                                }
                            },
                            404 => {
                                println!("City not found: {}", city);
                                error.set(Some(
                                    format!("City '{}' not found. Please check the spelling and try again.", city)
                                ))
                            },
                            429 => {
                                println!("Rate limited");
                                rate_limited.set(true);
                                error.set(Some(
                                    "Rate limit reached. Please wait 5 seconds between requests.".to_string()
                                ));
                            },
                            status => {
                                println!("Unexpected status code: {}", status);
                                error.set(Some(
                                    format!("Unexpected error occurred (Status: {}). Please try again.", status)
                                ));
                            }
                        }
                    },
                    Err(e) => {
                        println!("Network error: {:?}", e);
                        error.set(Some(
                            "Could not connect to weather service. Please check your internet connection.".to_string()
                        ));
                    }
                }
                loading.set(false);
            });
        })
    };

    let oninput = {
        let search_input = search_input.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            search_input.set(input.value());
        })
    };

    let toggle_theme = {
        let is_dark = is_dark.clone();
        Callback::from(move |_| {
            let new_value = !*is_dark;
            utils::toggle_dark_mode(new_value);
            is_dark.set(new_value);
        })
    };

    html! {
        <div class={Styles::CONTAINER}>
            <div class={Styles::CONTENT_WRAPPER}>
                <SearchForm 
                    onsubmit={onsubmit}
                    oninput={oninput}
                    toggle_theme={toggle_theme}
                    search_input={(*search_input).clone()}
                    is_dark={*is_dark}
                    loading={*loading}
                />
                
                if *loading {
                    <div class={Styles::CARD_BASE}>
                        <div class={Styles::FLEX_CENTER}>
                            <div class={classes!(Styles::LOADING_DOT, "[animation-delay:-0.3s]")} />
                            <div class={classes!(Styles::LOADING_DOT, "[animation-delay:-0.15s]")} />
                            <div class={Styles::LOADING_DOT} />
                        </div>
                    </div>
                }

                if let Some(err) = (*error).clone() {
                    <ErrorDisplay 
                        error={Some(err)}
                        rate_limited={*rate_limited}
                    />
                }

                if let Some(weather) = (*weather_data).clone() {
                    <WeatherDisplay weather={weather} />
                }
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}