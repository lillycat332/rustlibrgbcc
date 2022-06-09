#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rgb_to() {
        println!();
        let cols = vec![
            Color {
                red:   rand::random::<u8>(),
                green: rand::random::<u8>(),
                blue:  rand::random::<u8>(),
            },

            Color {
                red:   rand::random::<u8>(),
                green: rand::random::<u8>(),
                blue:  rand::random::<u8>(),
            },

            Color {
                red:   rand::random::<u8>(),
                green: rand::random::<u8>(),
                blue:  rand::random::<u8>(),
            },
        ];
        for col in cols {
            println!("{} Hello, World! {}", rgb_fg(col), reset())
        }
        println!();
    }

    #[test]
    fn hex_to() {
        println!();
        let hexes = vec![
            "#000000",
            "#ffffff",
            "#ff0000",
            "#00ff00",
            "#0000ff",
            "#ffff00",
            "#00ffff",
            "#ff00ff",
        ];

        for hex in hexes {
            println!("{}", hex_cc_fg(hex, "Hello, World!"));
        }
        println!();
    }
}

let reset = "\x1b[0m".to_string()

fn hex_to_rgb(hex: &str) -> Color {
    let hex = hex.trim_start_matches("#");
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    Color { red: r, green: g, blue: b }
}

pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

pub fn rgb_fg(color: Color, body: &str) -> String {
    format!("\x1b[38;2;{};{};{}m{}{}", color.red, color.green, color.blue, body, reset)
}

pub fn hex_fg(color: &str, body: &str) -> String {
    return format!("{}{}{}", color_cc_fg(hex_to_rgb(color)), body, reset());
}