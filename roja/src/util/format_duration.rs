const SECS_IN_MIN: i64 = 60;

fn pad_zero(unit: i64) -> String {
    if unit < 10 {
        format!("0{}", unit)
    } else {
        unit.to_string()
    }
}

pub fn format_duration(secs: i64) -> String {
    let negative = if secs < 0 { "-" } else { "" };
    let secs = if secs < 0 { -secs } else { secs };
    let mins = secs / SECS_IN_MIN;
    let secs = secs - (mins * SECS_IN_MIN);

    // truncate mins to 2 characters, because what song is >99 minutes?
    let mut mins = mins.to_string();
    mins.truncate(2);

    format!("{}{}:{}", negative, mins, pad_zero(secs))
}
