const SECS_IN_HOUR: i64 = 3600;
const SECS_IN_MIN: i64 = 60;

fn pad(unit: i64) -> String {
    if unit < 10 {
        format!("0{}", unit)
    } else {
        unit.to_string()
    }
}

pub fn format_duration(secs: i64) -> String {
    let negative = if secs < 0 { "-" } else { "" };
    let secs = if secs < 0 { -secs } else { secs };
    let hours = secs / SECS_IN_HOUR;
    let mins = (secs - (hours * SECS_IN_HOUR)) / SECS_IN_MIN;
    let secs = secs - (hours * SECS_IN_HOUR) - (mins * SECS_IN_MIN);

    if hours == 0 {
        format!("{}{}:{}", negative, pad(mins), pad(secs))
    } else {
        format!("{}{}:{}:{}", negative, pad(hours), pad(mins), pad(secs))
    }
}
