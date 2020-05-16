use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use super::usize_to_u16;

pub fn truncate(text: &str, target_width: u16) -> (&str, u16) {
    let (index, display_width) = find_truncated_string_end(text, target_width);
    (&text[..index], display_width)
}

fn find_truncated_string_end(text: &str, target_width: u16) -> (usize, u16) {
    let mut display_width = 0;

    for (i, c) in text.grapheme_indices(true) {
        let cwidth = usize_to_u16(UnicodeWidthStr::width(c));
        let next_display_width = display_width + cwidth;

        if next_display_width > target_width {
            return (i, display_width);
        }

        display_width = next_display_width;
    }

    (text.len(), display_width)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_truncate() {
        assert_eq!(truncate("", 10), ("", 0));
        assert_eq!(truncate("a", 10), ("a", 1));
        assert_eq!(truncate("abcde", 10), ("abcde", 5));
        assert_eq!(truncate("はじめての", 5), ("はじ", 4));
    }
}
