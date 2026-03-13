use ratatui::style::Color;

pub fn hex_to_color(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    if hex.len() == 6
        && let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&hex[0..2], 16),
            u8::from_str_radix(&hex[2..4], 16),
            u8::from_str_radix(&hex[4..6], 16),
        )
    {
        return Color::Rgb(r, g, b);
    }
    Color::White
}

pub fn supports_truecolor() -> bool {
    // Standard: COLORTERM is the canonical indicator (Linux/macOS)
    if std::env::var("COLORTERM")
        .map(|v| v == "truecolor" || v == "24bit")
        .unwrap_or(false)
    {
        return true;
    }

    // Windows Terminal sets WT_SESSION
    if std::env::var("WT_SESSION").is_ok() {
        return true;
    }

    // ConEmu/Cmder set ConEmuANSI=ON
    if std::env::var("ConEmuANSI")
        .map(|v| v == "ON")
        .unwrap_or(false)
    {
        return true;
    }

    // TERM_PROGRAM covers VS Code integrated terminal, iTerm2, WezTerm
    if let Ok(tp) = std::env::var("TERM_PROGRAM")
        && (tp == "vscode" || tp == "iTerm.app" || tp == "WezTerm")
    {
        return true;
    }

    // Known Linux terminal emulators that support truecolor
    if std::env::var("KITTY_WINDOW_ID").is_ok()
        || std::env::var("ALACRITTY_WINDOW_ID").is_ok()
        || std::env::var("FOOT_TERMINAL_VERSION").is_ok()
    {
        return true;
    }

    // Modern Windows terminals (ConPTY) support truecolor
    #[cfg(target_os = "windows")]
    {
        true
    }

    #[cfg(not(target_os = "windows"))]
    false
}

/// Convert an Rgb color to its nearest ANSI equivalent unconditionally.
/// Call site is responsible for checking `supports_truecolor()` before invoking.
pub fn to_256_fallback(color: Color) -> Color {
    match color {
        Color::Rgb(_, _, _) => approximate_ansi(color),
        _ => color,
    }
}

pub fn approximate_ansi(color: Color) -> Color {
    let Color::Rgb(r, g, b) = color else {
        return color;
    };

    let max_ch = r.max(g).max(b);
    let min_ch = r.min(g).min(b);

    // Near-gray: all channels within 10 of each other
    if max_ch - min_ch <= 10 {
        let avg = (r as u16 + g as u16 + b as u16) / 3;
        if avg < 4 {
            return Color::Indexed(16); // cube black
        }
        if avg > 246 {
            return Color::Indexed(231); // cube white
        }
        // Grayscale ramp: indices 232..=255 map to grays 8, 18, 28, ..., 238
        let idx = ((avg as f64 - 8.0) / 10.0).round().clamp(0.0, 23.0) as u8;
        return Color::Indexed(232 + idx);
    }

    // Color cube: map each channel to 0-5 index
    let r_idx = (r as f64 / 255.0 * 5.0).round() as u8;
    let g_idx = (g as f64 / 255.0 * 5.0).round() as u8;
    let b_idx = (b as f64 / 255.0 * 5.0).round() as u8;

    Color::Indexed(16 + 36 * r_idx + 6 * g_idx + b_idx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Color;

    // --- supports_truecolor ---

    #[test]
    fn test_supports_truecolor_with_truecolor_env() {
        unsafe { std::env::set_var("COLORTERM", "truecolor") };
        assert!(supports_truecolor());
        unsafe { std::env::remove_var("COLORTERM") };
    }

    #[test]
    fn test_supports_truecolor_with_24bit_env() {
        unsafe { std::env::set_var("COLORTERM", "24bit") };
        assert!(supports_truecolor());
        unsafe { std::env::remove_var("COLORTERM") };
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn test_supports_truecolor_unset_returns_false() {
        unsafe { std::env::remove_var("COLORTERM") };
        unsafe { std::env::remove_var("WT_SESSION") };
        unsafe { std::env::remove_var("ConEmuANSI") };
        unsafe { std::env::remove_var("TERM_PROGRAM") };
        unsafe { std::env::remove_var("KITTY_WINDOW_ID") };
        unsafe { std::env::remove_var("ALACRITTY_WINDOW_ID") };
        unsafe { std::env::remove_var("FOOT_TERMINAL_VERSION") };
        assert!(!supports_truecolor());
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_supports_truecolor_windows_default_returns_true() {
        unsafe { std::env::remove_var("COLORTERM") };
        unsafe { std::env::remove_var("WT_SESSION") };
        unsafe { std::env::remove_var("ConEmuANSI") };
        unsafe { std::env::remove_var("TERM_PROGRAM") };
        assert!(supports_truecolor());
    }

    #[test]
    fn test_supports_truecolor_wt_session() {
        unsafe { std::env::remove_var("COLORTERM") };
        unsafe { std::env::set_var("WT_SESSION", "some-guid") };
        assert!(supports_truecolor());
        unsafe { std::env::remove_var("WT_SESSION") };
    }

    #[test]
    fn test_supports_truecolor_conemu() {
        unsafe { std::env::remove_var("COLORTERM") };
        unsafe { std::env::remove_var("WT_SESSION") };
        unsafe { std::env::set_var("ConEmuANSI", "ON") };
        assert!(supports_truecolor());
        unsafe { std::env::remove_var("ConEmuANSI") };
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn test_supports_truecolor_conemu_off_returns_false() {
        unsafe { std::env::remove_var("COLORTERM") };
        unsafe { std::env::remove_var("WT_SESSION") };
        unsafe { std::env::remove_var("TERM_PROGRAM") };
        unsafe { std::env::set_var("ConEmuANSI", "OFF") };
        unsafe { std::env::remove_var("KITTY_WINDOW_ID") };
        unsafe { std::env::remove_var("ALACRITTY_WINDOW_ID") };
        unsafe { std::env::remove_var("FOOT_TERMINAL_VERSION") };
        assert!(!supports_truecolor());
        unsafe { std::env::remove_var("ConEmuANSI") };
    }

    #[test]
    fn test_supports_truecolor_term_program_vscode() {
        unsafe { std::env::remove_var("COLORTERM") };
        unsafe { std::env::remove_var("WT_SESSION") };
        unsafe { std::env::remove_var("ConEmuANSI") };
        unsafe { std::env::set_var("TERM_PROGRAM", "vscode") };
        assert!(supports_truecolor());
        unsafe { std::env::remove_var("TERM_PROGRAM") };
    }

    #[test]
    fn test_supports_truecolor_term_program_iterm() {
        unsafe { std::env::remove_var("COLORTERM") };
        unsafe { std::env::remove_var("WT_SESSION") };
        unsafe { std::env::remove_var("ConEmuANSI") };
        unsafe { std::env::set_var("TERM_PROGRAM", "iTerm.app") };
        assert!(supports_truecolor());
        unsafe { std::env::remove_var("TERM_PROGRAM") };
    }

    #[test]
    fn test_supports_truecolor_term_program_wezterm() {
        unsafe { std::env::remove_var("COLORTERM") };
        unsafe { std::env::remove_var("WT_SESSION") };
        unsafe { std::env::remove_var("ConEmuANSI") };
        unsafe { std::env::set_var("TERM_PROGRAM", "WezTerm") };
        assert!(supports_truecolor());
        unsafe { std::env::remove_var("TERM_PROGRAM") };
    }

    // --- terminal emulator env var detection ---

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn test_supports_truecolor_kitty() {
        unsafe { std::env::remove_var("COLORTERM") };
        unsafe { std::env::remove_var("WT_SESSION") };
        unsafe { std::env::remove_var("ConEmuANSI") };
        unsafe { std::env::remove_var("TERM_PROGRAM") };
        unsafe { std::env::remove_var("ALACRITTY_WINDOW_ID") };
        unsafe { std::env::remove_var("FOOT_TERMINAL_VERSION") };
        unsafe { std::env::set_var("KITTY_WINDOW_ID", "1") };
        assert!(supports_truecolor());
        unsafe { std::env::remove_var("KITTY_WINDOW_ID") };
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn test_supports_truecolor_alacritty() {
        unsafe { std::env::remove_var("COLORTERM") };
        unsafe { std::env::remove_var("WT_SESSION") };
        unsafe { std::env::remove_var("ConEmuANSI") };
        unsafe { std::env::remove_var("TERM_PROGRAM") };
        unsafe { std::env::remove_var("KITTY_WINDOW_ID") };
        unsafe { std::env::remove_var("FOOT_TERMINAL_VERSION") };
        unsafe { std::env::set_var("ALACRITTY_WINDOW_ID", "1") };
        assert!(supports_truecolor());
        unsafe { std::env::remove_var("ALACRITTY_WINDOW_ID") };
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn test_supports_truecolor_foot() {
        unsafe { std::env::remove_var("COLORTERM") };
        unsafe { std::env::remove_var("WT_SESSION") };
        unsafe { std::env::remove_var("ConEmuANSI") };
        unsafe { std::env::remove_var("TERM_PROGRAM") };
        unsafe { std::env::remove_var("KITTY_WINDOW_ID") };
        unsafe { std::env::remove_var("ALACRITTY_WINDOW_ID") };
        unsafe { std::env::set_var("FOOT_TERMINAL_VERSION", "1.17.0") };
        assert!(supports_truecolor());
        unsafe { std::env::remove_var("FOOT_TERMINAL_VERSION") };
    }

    // --- to_256_fallback ---

    #[test]
    fn test_to_256_fallback_passes_through_non_rgb() {
        assert_eq!(to_256_fallback(Color::Red), Color::Red);
        assert_eq!(to_256_fallback(Color::Blue), Color::Blue);
        assert_eq!(to_256_fallback(Color::White), Color::White);
    }

    #[test]
    fn test_to_256_fallback_always_converts_rgb() {
        let result = to_256_fallback(Color::Rgb(255, 0, 0));
        assert!(!matches!(result, Color::Rgb(_, _, _)));
    }

    // --- approximate_ansi ---

    #[test]
    fn test_approximate_ansi_pure_red() {
        let result = approximate_ansi(Color::Rgb(255, 0, 0));
        assert_eq!(result, Color::Indexed(196));
    }

    #[test]
    fn test_approximate_ansi_pure_green() {
        let result = approximate_ansi(Color::Rgb(0, 255, 0));
        assert_eq!(result, Color::Indexed(46));
    }

    #[test]
    fn test_approximate_ansi_pure_blue() {
        let result = approximate_ansi(Color::Rgb(0, 0, 255));
        assert_eq!(result, Color::Indexed(21));
    }

    #[test]
    fn test_approximate_ansi_black() {
        let result = approximate_ansi(Color::Rgb(0, 0, 0));
        assert_eq!(result, Color::Indexed(16));
    }

    #[test]
    fn test_approximate_ansi_white() {
        let result = approximate_ansi(Color::Rgb(255, 255, 255));
        assert_eq!(result, Color::Indexed(231));
    }

    #[test]
    fn test_approximate_ansi_non_rgb_passthrough() {
        assert_eq!(approximate_ansi(Color::Cyan), Color::Cyan);
    }

    // --- 256-color regression tests ---

    #[test]
    fn test_approximate_ansi_cursor_colors_distinguishable() {
        // Regression: cursor (dark bg) and text (light fg) must not map to the same index.
        // Before the fix both collapsed to the same 16-color value, making the cursor invisible.
        let bg = approximate_ansi(Color::Rgb(26, 26, 46));
        let fg = approximate_ansi(Color::Rgb(224, 192, 151));
        assert_ne!(bg, fg, "cursor background and foreground must map to different indexed colors");
    }

    #[test]
    fn test_approximate_ansi_cube_mixed() {
        // Rgb(0, 95, 135): r_idx=0, g_idx=2, b_idx=3 => 16 + 36*0 + 6*2 + 3 = 31
        let result = approximate_ansi(Color::Rgb(0, 95, 135));
        assert_eq!(result, Color::Indexed(31));
    }

    #[test]
    fn test_approximate_ansi_grayscale_midgray() {
        // Rgb(118, 118, 118) is a neutral gray; must map into the 24-step grayscale ramp (232..=255).
        let result = approximate_ansi(Color::Rgb(118, 118, 118));
        assert!(
            matches!(result, Color::Indexed(i) if (232..=255).contains(&i)),
            "expected grayscale ramp index 232..=255, got {:?}",
            result
        );
    }

    // --- Valid hex ---

    #[test]
    fn test_hex_to_color_with_hash_prefix() {
        assert_eq!(hex_to_color("#ff0000"), Color::Rgb(255, 0, 0));
    }

    #[test]
    fn test_hex_to_color_without_hash_prefix() {
        assert_eq!(hex_to_color("00ff00"), Color::Rgb(0, 255, 0));
    }

    #[test]
    fn test_hex_to_color_blue() {
        assert_eq!(hex_to_color("#0000ff"), Color::Rgb(0, 0, 255));
    }

    #[test]
    fn test_hex_to_color_white() {
        assert_eq!(hex_to_color("#ffffff"), Color::Rgb(255, 255, 255));
    }

    #[test]
    fn test_hex_to_color_black() {
        assert_eq!(hex_to_color("#000000"), Color::Rgb(0, 0, 0));
    }

    #[test]
    fn test_hex_to_color_mixed_case() {
        // uppercase hex digits
        assert_eq!(hex_to_color("#FF8800"), Color::Rgb(255, 136, 0));
    }

    #[test]
    fn test_hex_to_color_arbitrary_color() {
        assert_eq!(hex_to_color("#1a2b3c"), Color::Rgb(0x1a, 0x2b, 0x3c));
    }

    // --- Invalid hex -> fallback to White ---

    #[test]
    fn test_hex_to_color_too_short_returns_white() {
        assert_eq!(hex_to_color("#fff"), Color::White);
    }

    #[test]
    fn test_hex_to_color_empty_string_returns_white() {
        assert_eq!(hex_to_color(""), Color::White);
    }

    #[test]
    fn test_hex_to_color_invalid_chars_returns_white() {
        assert_eq!(hex_to_color("#zzzzzz"), Color::White);
    }

    #[test]
    fn test_hex_to_color_too_long_returns_white() {
        assert_eq!(hex_to_color("#ff000000"), Color::White);
    }

    #[test]
    fn test_hex_to_color_seven_chars_no_hash_returns_white() {
        // 7 chars without leading #: len after strip is 7, not 6
        assert_eq!(hex_to_color("ff00000"), Color::White);
    }
}
