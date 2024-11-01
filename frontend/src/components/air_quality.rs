use yew::prelude::*;
use crate::styles::Styles;
use weather_app_shared::WeatherData;

#[derive(Properties, PartialEq)]
struct AqiMetricProps {
    name: &'static str,
    value: Option<f64>,
    unit: &'static str,
}

#[function_component(AqiMetric)]
fn aqi_metric(props: &AqiMetricProps) -> Html {
    let formatted_value = props.value
        .map_or_else(|| "N/A".to_owned(), |v| format!("{:.1}{}", v, props.unit));

    html! {
        <div class="flex justify-between items-center py-2 border-b border-stone-200 dark:border-stone-700 last:border-0">
            <span class={classes!(Styles::METRIC_LABEL, "w-16")}>{props.name}</span>
            <span class={classes!(Styles::METRIC_VALUE, "text-right", "whitespace-nowrap", "flex-shrink-0")}>
                {formatted_value}
            </span>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct AirQualityProps {
    pub weather: WeatherData,
}

#[function_component(AirQuality)]
pub fn air_quality(props: &AirQualityProps) -> Html {
    let aqi = &props.weather.current.air_quality;
    
    let (aqi_color, aqi_label) = if let Some(aqi) = aqi.as_ref() {
        match aqi.us_epa_index {
            Some(1) => ("text-green-400", "Good"),
            Some(2) => ("text-yellow-400", "Moderate"),
            Some(3) => ("text-orange-400", "Unhealthy for Sensitive Groups"),
            Some(4) => ("text-red-400", "Unhealthy"),
            Some(5) => ("text-purple-400", "Very Unhealthy"),
            Some(6) => ("text-red-600", "Hazardous"),
            _ => ("text-stone-900 dark:text-stone-50", "Unknown")
        }
    } else {
        ("text-stone-900 dark:text-stone-50", "Unknown")
    };

    html! {
        <div class={Styles::METRIC_CARD}>
            <h3 class={classes!(Styles::HEADING_3, "mb-4")}>{"Air Quality"}</h3>
            if let Some(aqi) = aqi {
                <div class="space-y-4">
                    <div class="mb-6 pb-4 border-b border-stone-200 dark:border-stone-700">
                        <p class={classes!(Styles::METRIC_LABEL, "mb-2")}>{"US EPA Index"}</p>
                        <div class="flex justify-between items-center">
                            <p class={classes!(Styles::AQI_VALUE, aqi_color)}>{aqi_label}</p>
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
                <p class={Styles::SECONDARY_TEXT}>{"Air quality data not available"}</p>
            }
        </div>
    }
}