use yew::prelude::*;
use crate::styles::Styles;
use crate::utils::format_local_time;
use crate::components::{WeatherMetrics, AirQuality};
use weather_app_shared::WeatherData;

#[derive(Properties, PartialEq)]
pub struct WeatherDisplayProps {
    pub weather: WeatherData,
}

#[function_component(WeatherDisplay)]
pub fn weather_display(props: &WeatherDisplayProps) -> Html {
    html! {
        <div class={classes!(Styles::CARD_BASE, "max-w-4xl", "mx-auto")}>
            <div class={Styles::GRID_CONTAINER}>
                <div class={Styles::MAIN_CONTENT}>
                    <div class="flex justify-between items-start ml-1 mb-2">
                        <div>
                            <h2 class={Styles::HEADING_2}>
                                {&props.weather.location.name}
                                <span class="text-sm text-stone-400 ml-2">
                                    {format!("{}", props.weather.location.country)}
                                </span>
                            </h2>
                            <p class={Styles::COORDS_TEXT}>
                                {format!("Lat: {:.2}°, Lon: {:.2}°", 
                                    props.weather.location.lat, 
                                    props.weather.location.lon
                                )}
                            </p>
                            <p class={Styles::SECONDARY_TEXT}>
                                {format_local_time(&props.weather.location.localtime)}
                            </p>
                        </div>
                    </div>

                    <div class={classes!(Styles::WEATHER_INFO, "p-6", "mb-6")}>
                        <div class="flex items-center justify-between">
                            <div class="flex items-center gap-4">
                                <img 
                                    src={props.weather.current.condition.icon.clone()} 
                                    alt="Weather icon" 
                                    class={Styles::WEATHER_ICON}
                                />
                                <div>
                                    <div class="flex flex-col">
                                        <span class={Styles::WEATHER_TEMP}>
                                            {format!("{:.1}°C", props.weather.current.temp_c)}
                                        </span>
                                        <span class={Styles::TEMP_UNIT}>
                                            {format!("({:.1}°F)", props.weather.current.temp_f)}
                                        </span>
                                    </div>
                                    <p class={Styles::CONDITION_TEXT}>
                                        {&props.weather.current.condition.text}
                                    </p>
                                </div>
                            </div>
                            <div class="text-right">
                                <p class={Styles::SECONDARY_TEXT}>{"Feels like"}</p>
                                <p class={Styles::FEELS_LIKE}>
                                    {format!("{:.1}°C ({:.1}°F)", 
                                        props.weather.current.feelslike_c,
                                        props.weather.current.feelslike_f
                                    )}
                                </p>
                            </div>
                        </div>
                    </div>

                    <WeatherMetrics weather={props.weather.clone()} />
                </div>

                <div class="lg:col-span-1">
                    <AirQuality weather={props.weather.clone()} />
                </div>
            </div>

            <div class="text-center mt-4">
                <p class={classes!(Styles::SECONDARY_TEXT, "text-xs", "italic")}>
                    {"Source: "}
                    <a 
                        href="https://www.weatherapi.com" 
                        target="_blank" 
                        rel="noopener noreferrer"
                        class="hover:text-orange-500 transition-colors duration-200"
                    >
                        {"weatherapi.com"}
                    </a>
                </p>
            </div>
        </div>
    }
}