use std::rc::Rc;
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

#[derive(Copy, Clone, PartialEq)]
pub struct Specificity {
    pub a: u32,
    pub b: u32,
    pub c: u32,
}

impl PartialOrd for Specificity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.a == other.a {
            if self.b == other.b {
                self.c.partial_cmp(&other.c)
            } else {
                self.b.partial_cmp(&other.b)
            }
        } else {
            self.a.partial_cmp(&other.a)
        }
    }
}

impl std::ops::Add for Specificity {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Specificity {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
            c: self.c + rhs.c,
        }
    }
}

impl Specificity {
    pub fn new(a: u32, b: u32, c: u32) -> Specificity {
        Specificity {
            a,
            b,
            c,
        }
    }

    pub fn add_id(&mut self) {
        self.a += 1;
    }
    pub fn add_class(&mut self) {
        self.b += 1;
    }
    pub fn add_classes(&mut self, n: u32) {
        self.b += n;
    }
    pub fn add_tag(&mut self) {
        self.c += 1;
    }
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
    
    pub fn specificity(&self) -> Specificity {
        let mut specificity = Specificity::new(0, 0, 0);
        if self.id.is_some() {
            specificity.add_id();
        }
        specificity.add_classes(self.class_list.len() as u32);
        if self.tag_name.is_some() {
            specificity.add_tag();
        }
        specificity
    }

    pub fn matches(&self, tag_name: &Option<String>, class_list: &Vec<String>, id: &Option<String>, _node_handle: Option<tl::NodeHandle>) -> bool {
        if self.tag_name.is_some() && tag_name.is_some() && self.tag_name.as_ref().unwrap() != tag_name.as_ref().unwrap() {
            return false;
        }
        if self.id.is_some() && self.id.as_ref().unwrap() != id.as_ref().unwrap() {
            return false;
        }
        for class in &self.class_list {
            if !class_list.contains(class) {
                return false;
            }
        }
        true
    }
}

pub struct CssParseError;

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

pub struct Style {
    pub selectors: Vec<Selector>,
    pub properties: HashMap<String, String>,
}

impl Style {
    pub fn get_value<T>(&self, property: &str) -> (Option<T>, Option<Unit>) where T: std::str::FromStr {
        let value = self.properties.get(property);
        if value.is_none() {
            return (None, None);
        }
        let value = value.unwrap();
        for unit in vec!["px", "pt", "em", "%", "vw", "vh"] {
            if value.ends_with(unit) {
                let val = value[0..value.len() - unit.len()].parse::<T>().ok();
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
        return (value.parse::<T>().ok(), None);
    }
    pub fn set_value(&mut self, property: &str, value: &str) {
        self.properties.insert(property.to_string(), value.to_string());
    }

    pub fn get_matching_selector_with_highest_specificity(&self, tag_name: &Option<String>, class_list: &Vec<String>, id: &Option<String>, node_handle: Option<tl::NodeHandle>) -> Option<&Selector> {
        let mut selected_selector: Option<&Selector> = None;
        for selector in &self.selectors {
            if selector.matches(tag_name, class_list, id, node_handle) {
                if selected_selector.is_none() || selector.specificity() > selected_selector.unwrap().specificity() {
                    selected_selector = Some(selector);
                }
            }
        }
        selected_selector
    }
}

pub struct SelectedStyle {
    pub specificity: Specificity,
    pub style: Rc<Style>,
}

impl SelectedStyle {
    pub fn get_value<T>(&self, property: &str) -> (Option<T>, Option<Unit>) where T: std::str::FromStr {
        self.style.get_value(property)
    }
}

pub struct ComputedStyle {
    pub node_handle: tl::NodeHandle,
    pub selector: Selector,
    pub properties: HashMap<String, SelectedStyle>,
}

impl ComputedStyle {
    pub fn new(node_handle: tl::NodeHandle, selector: Selector) -> ComputedStyle {
        ComputedStyle {
            node_handle,
            selector,
            properties: HashMap::new()
        }
    }

    pub fn get_value<T>(&self, property: &str) -> (Option<T>, Option<Unit>) where T: std::str::FromStr {
        match self.properties.get(property) {
            Some(property_style) => property_style.get_value(property),
            None => (None, None)
        }
    }

    pub fn apply_style(&mut self, style: Rc<Style>) {
        match style.get_matching_selector_with_highest_specificity(&self.selector.tag_name, &self.selector.class_list, &self.selector.id, Some(self.node_handle)) {
            Some(selected_selector) => {
                let selected_specificity = selected_selector.specificity();
                for (property, _value) in style.properties.iter() {
                    match self.properties.get(property) {
                        Some(old_style) => {
                            if selected_specificity > old_style.specificity {
                                self.properties.insert(property.to_string(), SelectedStyle { 
                                    specificity: selected_specificity,
                                    style: style.clone()
                                });
                            }
                        },
                        None => {
                            self.properties.insert(property.to_string(), SelectedStyle { 
                                specificity: selected_specificity,
                                style: style.clone()
                            });
                        }
                    }
                }
            },
            None => {}
        }
    }
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
