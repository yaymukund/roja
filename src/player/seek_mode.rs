pub enum SeekMode {
    Relative,
    Absolute,
    // AbsolutePercent,
    // RelativePercent,
    // Keyframes,
    // Exact,
}

impl SeekMode {
    pub fn as_str(&self) -> &str {
        match self {
            SeekMode::Relative => "relative",
            SeekMode::Absolute => "absolute",
            // SeekMode::AbsolutePercent => "absolute-percent",
            // SeekMode::RelativePercent => "relative-percent",
            // SeekMode::Keyframes => "keyframes",
            // SeekMode::Exact => "exact",
        }
    }
}
