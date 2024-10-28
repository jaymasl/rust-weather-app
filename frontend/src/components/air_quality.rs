use yew::prelude::*;
use crate::styles::Styles;
use weather_app_shared::WeatherData;

#[derive(Properties, PartialEq)]
struct AqiMetricProps {
    name: String,
    value: f64,
    unit: String,
}

#[function_component(AqiMetric)]
fn aqi_metric(props: &AqiMetricProps) -> Html {
    html! {
        <div class="flex justify-between items-center py-2 border-b border-stone-200 dark:border-stone-700 last:border-0">
            <span class={classes!(Styles::METRIC_LABEL, "w-16")}>
                {&props.name}
            </span>
            <span class={classes!(Styles::METRIC_VALUE, "text-right", "whitespace-nowrap", "flex-shrink-0")}>
                {format!("{:.1}{}", props.value, props.unit)}
            </span>
        </div>
    }
}

fn get_aqi_label(index: i32) -> &'static str {
    match index {
        1 => "Good",
        2 => "Moderate",
        3 => "Unhealthy for Sensitive Groups",
        4 => "Unhealthy",
        5 => "Very Unhealthy",
        6 => "Hazardous",
        _ => "Unknown",
    }
}

#[derive(Properties, PartialEq)]
pub struct AirQualityProps {
    pub weather: WeatherData,
}

#[function_component(AirQuality)]
pub fn air_quality(props: &AirQualityProps) -> Html {
    html! {
        <div class={Styles::METRIC_CARD}>
            <h3 class={classes!(Styles::HEADING_3, "mb-4")}>{"Air Quality"}</h3>
            if let Some(aqi) = &props.weather.current.air_quality {
                <div class="space-y-4">
                    <div class="mb-6 pb-4 border-b border-stone-200 dark:border-stone-700">
                        <p class={classes!(Styles::METRIC_LABEL, "mb-2")}>{"US EPA Index"}</p>
                        <div class="flex justify-between items-center">
                            <p class={classes!(
                                Styles::AQI_VALUE,
                                match aqi.us_epa_index {
                                    1 => "text-green-400",
                                    2 => "text-yellow-400",
                                    3 => "text-orange-400",
                                    4 => "text-red-400",
                                    5 => "text-purple-400",
                                    6 => "text-red-600",
                                    _ => "text-stone-900 dark:text-stone-50"
                                }
                            )}>
                                {get_aqi_label(aqi.us_epa_index)}
                            </p>
                        </div>
                    </div>
                    <div class="space-y-0.5">
                        <AqiMetric name="PM2.5" value={aqi.pm2_5} unit=" µg/m³" />
                        <AqiMetric name="PM10" value={aqi.pm10} unit=" µg/m³" />
                        <AqiMetric name="O₃" value={aqi.o3} unit=" µg/m³" />
                        <AqiMetric name="NO₂" value={aqi.no2} unit=" µg/m³" />
                        <AqiMetric name="CO" value={aqi.co} unit=" µg/m³" />
                        <AqiMetric name="SO₂" value={aqi.so2} unit=" µg/m³" />
                    </div>
                </div>
            } else {
                <p class={Styles::SECONDARY_TEXT}>
                    {"Air quality data not available"}
                </p>
            }
        </div>
    }
}