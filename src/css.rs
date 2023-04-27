use std::str::FromStr;
use std::collections::HashMap;

use cssparser::BasicParseError;

pub enum Unit {
    Px,
    Pt,
    Em,
    Percent,
    Vw,
    Vh
}

#[derive(Clone)]
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

#[derive(Clone)]
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
    pub properties: HashMap<String, String>,
}

/// Parses styles from css.
/// 
/// Supports: Basic selectors (tag, multiple classes, id), Multiple selectors for one style, Units, Hex colors
/// 
/// TODO: Pseudo-classes, Pseudo-elements, Attribute selectors, Combinators, Media queries, Keyframes, Animations, Transitions, Variables, Functions, Calc, etc.
/// 
/// TODO: needs better error handling
pub fn parse_css(css: &str)-> Vec<Style> {
    let mut parser_input = cssparser::ParserInput::new(css);
    let mut parser = cssparser::Parser::new(&mut parser_input);
    let mut sheet: Vec<Style> = Vec::new();
    let mut selectors: Vec<Selector> = Vec::new();
    let mut current_selector = Selector::new(None, Vec::new(), None);
    loop {
        match parser.next() {
            Ok(token) => {
                match token {
                    cssparser::Token::Ident(ident) => {
                        if current_selector.tag_name.is_none() {
                            current_selector.tag_name = Some(ident.to_string());
                        } else {
                            current_selector.class_list.push(ident.to_string());
                        }
                    },
                    cssparser::Token::Hash(hash) => {
                        current_selector.id = Some(hash.to_string());
                    },
                    cssparser::Token::Comma => {
                        selectors.push(current_selector.clone());
                        current_selector = Selector::new(None, Vec::new(), None);
                    },
                    cssparser::Token::CurlyBracketBlock => {
                        // Parse Style
                        let style_result = parser.parse_nested_block(|block_parser| {
                            let mut style = Style {
                                selectors: Vec::new(),
                                properties: HashMap::new(),
                            };
                            style.selectors.append(selectors.as_mut());
                            let mut property_name = String::new();
                            let mut property_value = String::new();
                            let mut in_property_value = false;
                            loop {
                                match block_parser.next() {
                                    Ok(token) => {
                                        match token {
                                            cssparser::Token::Ident(ident) => {
                                                if in_property_value {
                                                    property_value.push_str(&ident.to_string());
                                                } else {
                                                    property_name.push_str(&ident.to_string());
                                                }
                                            },
                                            cssparser::Token::Colon => {
                                                in_property_value = true;
                                            },
                                            cssparser::Token::Dimension { has_sign, value, int_value, unit } => {
                                                if *has_sign {
                                                    property_value.push('-');
                                                }
                                                match int_value {
                                                    Some(int_value) => property_value.push_str(&int_value.to_string()),
                                                    None => property_value.push_str(&value.to_string())
                                                }
                                                property_value.push_str(&unit.to_string());
                                            },
                                            cssparser::Token::Percentage { has_sign, unit_value, int_value } => {
                                                if *has_sign {
                                                    property_value.push('-');
                                                }
                                                match int_value {
                                                    Some(int_value) => property_value.push_str(&int_value.to_string()),
                                                    None => property_value.push_str(&((*unit_value * 100.0) as i32).to_string())
                                                }
                                                property_value.push('%');
                                            },
                                            cssparser::Token::Number { has_sign, value, int_value } => {
                                                // Parse property value
                                                if *has_sign {
                                                    property_value.push('-');
                                                }
                                                match int_value {
                                                    Some(int_value) => property_value.push_str(&int_value.to_string()),
                                                    None => property_value.push_str(&value.to_string())
                                                }
                                            },
                                            cssparser::Token::Hash(hash) => {
                                                property_value.push('#');
                                                property_value.push_str(&hash.to_string());
                                            },
                                            cssparser::Token::QuotedString(string) => {
                                                property_value.push_str(string.as_ref());
                                            },
                                            cssparser::Token::WhiteSpace(ws) => {
                                                if in_property_value && property_value.len() > 0 {
                                                    property_value.push_str(*ws);
                                                }
                                            },
                                            cssparser::Token::Semicolon => {
                                                property_value = property_value.trim_end().to_string();
                                                style.properties.insert(property_name.clone(), property_value.clone());
                                                in_property_value = false;
                                                property_name.clear();
                                                property_value.clear();
                                            },
                                            // This error is only thrown so that rust can infer the error type
                                            cssparser::Token::BadString(_) => {
                                                return Err::<Style, cssparser::ParseError<'_, BasicParseError>>(cssparser::ParseError {
                                                    kind: cssparser::ParseErrorKind::Custom(cssparser::BasicParseError {
                                                        kind: cssparser::BasicParseErrorKind::UnexpectedToken(token.clone()),
                                                        location: block_parser.current_source_location(),
                                                    }),
                                                    location: block_parser.current_source_location(),
                                                });
                                            },
                                            _ => {}
                                        }
                                    },
                                    Err(_parse_error) => return Ok(style)
                                }
                            }
                        });
                        match style_result {
                            Ok(style) => {
                                sheet.push(style);
                            },
                            Err(parse_error) => {
                                println!("Error parsing css style: {:?}", parse_error);
                                break;
                            }
                        }
                    },
                    cssparser::Token::CloseCurlyBracket => {},
                    _ => {}
                }
            },
            Err(_) => break
        }
    }
    sheet
}
