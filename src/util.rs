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

    // truncate mins to 3 characters, because what song is >1000 minutes?
    let mut mins = mins.to_string();
    mins.truncate(3);

    format!("{}{}:{}", negative, mins, pad_zero(secs))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(0), "00:00");
        assert_eq!(format_duration(30), "00:30");
        assert_eq!(format_duration(-30), "-00:30");
        assert_eq!(format_duration(60), "01:00");
        assert_eq!(format_duration(-60), "-01:00");
        assert_eq!(format_duration(3600), "01:00:00");
        assert_eq!(format_duration(-3600), "-01:00:00");

        assert_eq!(format_duration(347), "05:47");
        assert_eq!(format_duration(34347), "09:32:27");
    }
}
