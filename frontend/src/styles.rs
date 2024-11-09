#[derive(Debug, Clone)]
pub struct Styles;

impl Styles {
    pub const CONTAINER: &'static str = "min-h-screen bg-white dark:bg-black transition-all duration-300";
    pub const CONTENT_WRAPPER: &'static str = "container mx-auto px-4 py-6 max-w-2xl transition-all duration-300";
    pub const GRID_CONTAINER: &'static str = "grid grid-cols-1 lg:grid-cols-3 gap-4";
    pub const MAIN_CONTENT: &'static str = "lg:col-span-2 space-y-4";
    pub const WEATHER_GRID: &'static str = "grid grid-cols-3 sm:grid-cols-3 gap-3";
    pub const FLEX_CENTER: &'static str = "flex justify-center items-center space-x-2";
    pub const CARD_BASE: &'static str = "bg-white dark:bg-stone-900 rounded-xl shadow-md p-5 mb-2 border border-stone-300 dark:border-stone-800 transition-all duration-300";
    pub const METRIC_CARD: &'static str = "bg-white dark:bg-stone-800 rounded-xl p-4 border border-stone-300 dark:border-stone-700 transition-all duration-300";
    pub const WEATHER_INFO: &'static str = "bg-white dark:bg-stone-800 rounded-xl p-4 border border-stone-300 dark:border-stone-700 transition-all duration-300";
    pub const HEADING_1: &'static str = "text-2xl font-bold text-stone-900 dark:text-stone-50 mb-1 text-center transition-colors duration-300";
    pub const HEADING_2: &'static str = "text-3xl font-semibold text-stone-900 dark:text-stone-50 transition-colors duration-300";
    pub const HEADING_3: &'static str = "text-lg font-semibold text-stone-900 dark:text-stone-50 mb-3 transition-colors duration-300";
    pub const SEARCH_INPUT: &'static str = "w-full p-3 bg-white dark:bg-stone-900 border border-stone-300 dark:border-stone-800 rounded-xl text-stone-900 dark:text-stone-50 placeholder-stone-400 dark:placeholder-stone-600 focus:ring-2 focus:ring-orange-500/50 outline-none transition-all duration-300";
    pub const SEARCH_BUTTON: &'static str = "w-full bg-orange-800 hover:bg-orange-500 active:bg-orange-800 text-white font-medium p-3 rounded-xl disabled:bg-stone-200 disabled:cursor-not-allowed transition-all duration-100";
    pub const THEME_TOGGLE: &'static str = "p-2.5 rounded-full bg-white dark:bg-stone-800 shadow-md hover:shadow-lg border border-stone-800 dark:border-stone-800 transition-all duration-100";
    pub const THEME_ICON: &'static str = "w-7 h-7 transition-transform duration-300 hover:scale-150";
    pub const WEATHER_ICON: &'static str = "w-25 h-25";
    pub const WEATHER_TEMP: &'static str = "text-4xl font-bold text-stone-900 dark:text-stone-50 transition-colors duration-300";
    pub const COORDS_TEXT: &'static str = "text-sm text-stone-600 dark:text-stone-400 transition-colors duration-300";
    pub const SECONDARY_TEXT: &'static str = "text-sm text-stone-600 dark:text-stone-400 transition-colors duration-300";
    pub const METRIC_LABEL: &'static str = "text-sm font-medium text-stone-600 dark:text-stone-400";
    pub const METRIC_VALUE: &'static str = "text-lg font-semibold text-stone-900 dark:text-stone-50";
    pub const AQI_VALUE: &'static str = "text-2lg font-semibold mt-2 transition-colors duration-300";
    pub const LOADING_DOT: &'static str = "w-4 h-4 bg-orange-500 rounded-full animate-bounce";
    pub const TEMP_UNIT: &'static str = "text-sm text-stone-600 dark:text-stone-400 transition-colors duration-300";
    pub const CONDITION_TEXT: &'static str = "text-base sm:text-lg text-stone-700 dark:text-stone-300 transition-colors duration-300";
    pub const FEELS_LIKE: &'static str = "text-sm text-stone-600 dark:text-stone-400 transition-colors duration-300";
}