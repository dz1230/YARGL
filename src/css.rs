use std::str::FromStr;


pub enum Unit {
    Px,
    Pt,
    Em,
    Percent,
    Vw,
    Vh
}

pub struct Selector {
    pub tag_name: Option<String>,
    pub class_list: Vec<String>,
    pub id: Option<String>,
}

impl Selector {
    pub fn new(tag_name: Option<String>, class_list: Vec<String>, id: Option<String>) -> Selector {
        Selector {
            tag_name,
            class_list,
            id,
        }
    }
}

pub struct CssParseError;

// FUCK RUST, why do I have to do this?
pub struct CssColor {
    pub sdl_color: sdl2::pixels::Color,
}

impl FromStr for CssColor {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        if chars.next() == Some('#') {
            let mut color: Vec<u8> = Vec::new();
            for _ in 0..3 {
                let mut hex: String = String::new();
                for _ in 0..2 {
                    match chars.next() {
                        Some(ch) => hex.push(ch),
                        None => return Err(CssParseError {})
                    }
                }
                match u8::from_str_radix(&hex, 16) {
                    Ok(num) => color.push(num),
                    Err(_) => return Err(CssParseError {})
                }
            }
            Ok(CssColor { sdl_color: sdl2::pixels::Color::RGB(color[0], color[1], color[2]) })
        } else {
            Err(CssParseError {})
        }
    }
    type Err = CssParseError;
}

pub struct Property {
    pub name: String,
    pub value: String,
}

impl Property {
    pub fn get_value<T>(&self) -> (Option<T>, Option<Unit>) where T: std::str::FromStr {
        for unit in vec!["px", "pt", "em", "%", "vw", "vh"] {
            if self.value.ends_with(unit) {
                let val = self.value[0..self.value.len() - unit.len()].parse::<T>().ok();
                let unit = match unit {
                    "px" => Some(Unit::Px),
                    "pt" => Some(Unit::Pt),
                    "em" => Some(Unit::Em),
                    "%" => Some(Unit::Percent),
                    "vw" => Some(Unit::Vw),
                    "vh" => Some(Unit::Vh),
                    _ => None,
                };
                return (val, unit);
            }
        }
        return (self.value.parse::<T>().ok(), None);
    }
}

pub struct Style {
    pub selectors: Vec<Selector>,
    pub properties: Vec<Property>,
}