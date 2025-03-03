use crate::plugin::Weather;

fn get_icon(weather: &Weather) -> &str {
    let is_day = weather.current.is_day == 1;
    match weather.current.condition.code {
        1000 => {
            if is_day {
                "☀️"
            } else {
                ""
            }
        } // Sunny/Clear
        1003 => {
            if is_day {
                "⛅️"
            } else {
                ""
            }
        } // Partly Cloudy/Partly Cloudy
        1006 => {
            if is_day {
                "🌥"
            } else {
                "🌥"
            }
        } // Cloudy/Cloudy
        1009 => {
            if is_day {
                "☁️"
            } else {
                "☁️"
            }
        } // Overcast/Overcast
        1030 => {
            if is_day {
                "🌫"
            } else {
                "🌫"
            }
        } // Mist/Mist
        1063 => {
            if is_day {
                "🌧"
            } else {
                "🌧"
            }
        } // Patchy rain possible/Patchy rain possible
        1066 => {
            if is_day {
                "🌨"
            } else {
                "🌨"
            }
        } // Patchy snow possible/Patchy snow possible
        1069 => {
            if is_day {
                "🌨"
            } else {
                "🌨"
            }
        } // Patchy sleet possible/Patchy sleet possible
        1072 => {
            if is_day {
                "🌨"
            } else {
                "🌨"
            }
        } // Patchy freezing drizzle possible/Patchy freezing drizzle possible
        1087 => {
            if is_day {
                "🌩"
            } else {
                "🌩"
            }
        } // Thundery outbreaks possible/Thundery outbreaks possible
        1114 => {
            if is_day {
                "🌨"
            } else {
                "🌨"
            }
        } // Blowing snow/Blowing snow
        1117 => {
            if is_day {
                "🌨"
            } else {
                "🌨"
            }
        } // Blizzard/Blizzard
        1135 => {
            if is_day {
                "🌫"
            } else {
                "🌫"
            }
        } // Fog/Fog
        1147 => {
            if is_day {
                "🌫"
            } else {
                "🌫"
            }
        } // Freezing fog/Freezing Fog
        1150 => {
            if is_day {
                "🌫"
            } else {
                "🌫"
            }
        } // Patchy light drizzle/Patchy light drizzle
        1153 => {
            if is_day {
                "🌫"
            } else {
                "🌫"
            }
        } // Light drizzle/Light drizzle
        1168 => {
            if is_day {
                "🌫"
            } else {
                "🌫"
            }
        } // Freezing drizzle/Freezing drizzle
        1171 => {
            if is_day {
                "🌫"
            } else {
                "🌫"
            }
        } // Heavy freezing drizzle/Heavy freezing drizzle
        1180 => {
            if is_day {
                "🌧"
            } else {
                "🌧"
            }
        } // Patchy light rain/Patchy light rain
        1183 => {
            if is_day {
                "🌧"
            } else {
                "🌧"
            }
        } // Light rain/Light rain
        1186 => {
            if is_day {
                "🌧"
            } else {
                "🌧"
            }
        } // Moderate rain at times/Moderate rain at times
        1189 => {
            if is_day {
                "🌧"
            } else {
                "🌧"
            }
        } // Moderate rain/Moderate rain
        1192 => {
            if is_day {
                "🌧"
            } else {
                "🌧"
            }
        } // Heavy rain at times/Heavy rain at times
        1195 => {
            if is_day {
                "🌧"
            } else {
                "🌧"
            }
        } // Heavy rain/Heavy rain
        1198 => {
            if is_day {
                "🌧"
            } else {
                "🌧"
            }
        } // Light freezing rain/Light freezing rain
        1201 => {
            if is_day {
                "🌧"
            } else {
                "🌧"
            }
        } // Moderate or heavy freezing rain/Moderate or heavy freezing rain
        1204 => {
            if is_day {
                "🌧"
            } else {
                "🌧"
            }
        } // Light sleet/Light sleet
        1207 => {
            if is_day {
                "🌧"
            } else {
                "🌧"
            }
        } // Moderate or heavy sleet/Moderate or heavy sleet
        1210 => {
            if is_day {
                "🌨"
            } else {
                "🌨"
            }
        } // Patchy light snow/Patchy light snow
        1213 => {
            if is_day {
                "🌨"
            } else {
                "🌨"
            }
        } // Light snow/Light snow
        1216 => {
            if is_day {
                "🌨"
            } else {
                "🌨"
            }
        } // Patchy moderate snow/Patchy moderate snow
        1219 => {
            if is_day {
                "🌨"
            } else {
                "🌨"
            }
        } // Moderate snow/Moderate snow
        1222 => {
            if is_day {
                "🌨"
            } else {
                "🌨"
            }
        } // Patchy heavy snow/Patchy heavy snow
        1225 => {
            if is_day {
                "🌨"
            } else {
                "🌨"
            }
        } // Heavy snow/Heavy snow
        1237 => {
            if is_day {
                "🌨"
            } else {
                "🌨"
            }
        } // Ice pellets/Ice pellets
        1240 => {
            if is_day {
                "🌧"
            } else {
                "🌧"
            }
        } // Light rain shower/Light rain shower
        1243 => {
            if is_day {
                "🌧"
            } else {
                "🌧"
            }
        } // Moderate or heavy rain shower/Moderate or heavy rain shower
        1246 => {
            if is_day {
                "🌨"
            } else {
                "🌨"
            }
        } // Torrential rain shower/Torrential rain shower
        1249 => {
            if is_day {
                "🌨"
            } else {
                "🌨"
            }
        } // Light sleet showers/Light sleet showers
        1252 => {
            if is_day {
                "🌨"
            } else {
                "🌨"
            }
        } // Moderate or heavy sleet showers/Moderate or heavy sleet showers
        1255 => {
            if is_day {
                "🌨"
            } else {
                "🌨"
            }
        } // Light snow showers/Light snow showers
        1258 => {
            if is_day {
                "🌨"
            } else {
                "🌨"
            }
        } // Moderate or heavy snow showers/Moderate or heavy snow showers
        1261 => {
            if is_day {
                "🌨"
            } else {
                "🌨"
            }
        } // Light showers of ice pellets/Light showers of ice pellets
        1264 => {
            if is_day {
                "🌨"
            } else {
                "🌨"
            }
        } // Moderate or heavy showers of ice pellets/Moderate or heavy showers of ice pellets
        1273 => {
            if is_day {
                "🌩"
            } else {
                "🌩"
            }
        } // Patchy light rain with thunder/Patchy light rain with Thunder
        1276 => {
            if is_day {
                "🌩"
            } else {
                "🌩"
            }
        } // Moderate or heavy rain with thunder/Moderate or heavy rain with Thunder
        1279 => {
            if is_day {
                "🌨"
            } else {
                "🌨"
            }
        } // Patchy light snow with thunder/Patchy light snow with Thunder
        1282 => {
            if is_day {
                "🌨"
            } else {
                "🌨"
            }
        } // Moderate or heavy snow with thunder/Moderate or heavy snow with Thunder
        _ => "❓",
    }
}

pub fn output_compact(weather: &Weather) -> String {
    let mut output = String::new();
    output.push_str(&format!("{}  ", get_icon(weather)));
    output.push_str(&format!("{}°C", weather.current.temp_c));
    output.push_str(&format!(" {}", weather.current.condition.text));
    output
}
