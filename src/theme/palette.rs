use ratatui::style::Color;

pub fn hex_to_color(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    if hex.len() == 6 {
        if let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&hex[0..2], 16),
            u8::from_str_radix(&hex[2..4], 16),
            u8::from_str_radix(&hex[4..6], 16),
        ) {
            return Color::Rgb(r, g, b);
        }
    }
    Color::White
}

pub fn supports_truecolor() -> bool {
    std::env::var("COLORTERM")
        .map(|v| v == "truecolor" || v == "24bit")
        .unwrap_or(false)
}

/// Map truecolor to nearest 256-color equivalent
pub fn to_256_fallback(color: Color) -> Color {
    match color {
        Color::Rgb(_, _, _) if !supports_truecolor() => {
            // Use basic ANSI colors as fallback
            approximate_ansi(color)
        }
        _ => color,
    }
}

fn approximate_ansi(color: Color) -> Color {
    let Color::Rgb(r, g, b) = color else {
        return color;
    };

    // Simple luminance-based mapping to basic 16 colors
    let lum = (r as u16 * 299 + g as u16 * 587 + b as u16 * 114) / 1000;
    let is_bright = lum > 128;

    // Determine dominant channel
    let max = r.max(g).max(b);
    if max < 40 {
        return Color::Black;
    }

    let r_dom = r > g && r > b;
    let g_dom = g > r && g > b;
    let b_dom = b > r && b > g;
    let rg = r > 100 && g > 100 && b < 80;
    let rb = r > 100 && b > 100 && g < 80;
    let gb = g > 100 && b > 100 && r < 80;

    if rg {
        if is_bright { Color::LightYellow } else { Color::Yellow }
    } else if rb {
        if is_bright { Color::LightMagenta } else { Color::Magenta }
    } else if gb {
        if is_bright { Color::LightCyan } else { Color::Cyan }
    } else if r_dom {
        if is_bright { Color::LightRed } else { Color::Red }
    } else if g_dom {
        if is_bright { Color::LightGreen } else { Color::Green }
    } else if b_dom {
        if is_bright { Color::LightBlue } else { Color::Blue }
    } else if lum > 200 {
        Color::White
    } else if lum > 100 {
        Color::Gray
    } else {
        Color::DarkGray
    }
}
