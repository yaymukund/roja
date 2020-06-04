use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// truncates the string, optionally with an ellipsis. fills any remaining space with empty string
pub fn fit_width(text: &str, width: usize, ellipsis: bool) -> String {
    if width == 0 {
        return String::new();
    }

    let mut rem = width;
    let mut out = String::with_capacity(rem);

    for grapheme in text.graphemes(true) {
        let gwidth = UnicodeWidthStr::width(grapheme);

        if rem == 0 {
            if ellipsis {
                out.pop();
                out.push('…');
            }

            return out;
        }

        if rem >= gwidth {
            rem -= gwidth;
            out.push_str(grapheme);
        } else {
            if ellipsis {
                out.push('…');
            }

            return out;
        }
    }

    for _ in 0..rem {
        out.push(' ');
    }

    out
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_fit_width() {
        assert_eq!(fit_width("", 10, true), "          ");
        assert_eq!(fit_width("a", 10, true), "a         ");
        assert_eq!(fit_width("abcde", 10, true), "abcde     ");
        assert_eq!(fit_width("はじめての", 5, true), "はじ…");
    }
}
