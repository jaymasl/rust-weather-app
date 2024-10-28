use yew::prelude::*;
use crate::styles::Styles;
use weather_app_shared::WeatherData;

#[derive(Properties, PartialEq)]
pub struct WeatherMetricsProps {
    pub weather: WeatherData,
}

#[function_component(WeatherMetrics)]
pub fn weather_metrics(props: &WeatherMetricsProps) -> Html {
    html! {
        <div class={Styles::WEATHER_GRID}>
            <div class={Styles::METRIC_CARD}>
                <div class="flex items-center gap-2 mb-2">
                    <svg class="w-5 h-5 text-stone-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3" />
                    </svg>
                    <h3 class={Styles::METRIC_LABEL}>{"Wind"}</h3>
                </div>
                <p class={Styles::METRIC_VALUE}>
                    {format!("{:.1} km/h", props.weather.current.wind_kph)}
                </p>
                <p class={Styles::SECONDARY_TEXT}>
                    {format!("{} {}", props.weather.current.wind_degree, props.weather.current.wind_dir)}
                </p>
            </div>

            <div class={Styles::METRIC_CARD}>
                <h3 class={Styles::METRIC_LABEL}>{"Humidity"}</h3>
                <p class={Styles::METRIC_VALUE}>
                    {format!("{}%", props.weather.current.humidity)}
                </p>
            </div>

            <div class={Styles::METRIC_CARD}>
                <h3 class={Styles::METRIC_LABEL}>{"Pressure"}</h3>
                <p class={Styles::METRIC_VALUE}>
                    {format!("{:.0} mb", props.weather.current.pressure_mb)}
                </p>
            </div>

            <div class={Styles::METRIC_CARD}>
                <h3 class={Styles::METRIC_LABEL}>{"Visibility"}</h3>
                <p class={Styles::METRIC_VALUE}>
                    {format!("{:.1} km", props.weather.current.vis_km)}
                </p>
            </div>

            <div class={Styles::METRIC_CARD}>
                <h3 class={Styles::METRIC_LABEL}>{"UV Index"}</h3>
                <p class={Styles::METRIC_VALUE}>
                    {format!("{:.0}", props.weather.current.uv)}
                </p>
            </div>

            <div class={Styles::METRIC_CARD}>
                <h3 class={Styles::METRIC_LABEL}>{"Precipitation"}</h3>
                <p class={Styles::METRIC_VALUE}>
                    {format!("{:.1} mm", props.weather.current.precip_mm)}
                </p>
            </div>
        </div>
    }
}