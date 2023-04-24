
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
    pub fn fromString(selector: &str) -> Selector {
        // TODO
    }
}

pub struct Property {
    pub name: String,
    pub value: String,
}

impl Property {
    pub fn get_number(&self) -> Option<f32> {
        let mut val = String::new();
        for c in self.value.as_str().chars() {
            if c < '0' || c > '9' && c != '.' {
                if val.len() == 0 {
                    continue;
                }
                break;
            } else {
                val.push(c);
            }
        }
        val.as_str().parse::<f32>().ok()
    }
    pub fn get_unit(&self) -> Option<Unit> {
        let mut unit = String::new();
        for c in self.value.as_str().chars().rev() {
            if (c < 'a' || c > 'z') && (c < 'A' || c > 'Z') && c != '%' {
                if unit.len() == 0 {
                    continue;
                }
                break;
            } else {
                unit.push(c);
            }
        }
        match unit.as_str() {
            "px" => Some(Unit::Px),
            "pt" => Some(Unit::Pt),
            "em" => Some(Unit::Em),
            "%" => Some(Unit::Percent),
            "vw" => Some(Unit::Vw),
            "vh" => Some(Unit::Vh),
            _ => None,
        }
    }
    pub fn get_color(&self) -> Option<sdl2::pixels::Color> {
        let mut chars = self.value.as_str().chars();
        if chars.next() == Some('#') {
            let mut hex = String::new();
            for _ in 0..6 {
                hex.push(chars.next()?);
            }
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some(sdl2::pixels::Color::RGB(r, g, b))
        } else {
            None
        }
    }
}

pub struct Style {
    pub selector: Selector,
    pub properties: Vec<Property>,
}