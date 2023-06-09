use std::rc::Rc;
use std::str::FromStr;
use std::collections::HashMap;

use cssparser::BasicParseError;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// Error which occured while parsing a css.
#[derive(Debug, PartialEq, Clone)]
pub struct CssParseError {
    pub css: String,
    pub message: String,
}

/// Unit of a css value.
#[derive(Debug, PartialEq, EnumIter)]
pub enum Unit {
    Px,
    Pt,
    Em,
    Percent,
    Vw,
    Vh,
    Vmin,
    Vmax,
    Cm,
    Mm,
    Q,
    In,
    Pc,
    Fr,
}

impl ToString for Unit {
    fn to_string(&self) -> String {
        match self {
            Unit::Px => "px".to_string(),
            Unit::Pt => "pt".to_string(),
            Unit::Em => "em".to_string(),
            Unit::Percent => "%".to_string(),
            Unit::Vw => "vw".to_string(),
            Unit::Vh => "vh".to_string(),
            Unit::Vmin => "vmin".to_string(),
            Unit::Vmax => "vmax".to_string(),
            Unit::Cm => "cm".to_string(),
            Unit::Mm => "mm".to_string(),
            Unit::Q => "Q".to_string(),
            Unit::In => "in".to_string(),
            Unit::Pc => "pc".to_string(),
            Unit::Fr => "fr".to_string(),
        }
    }
}

impl FromStr for Unit {
    type Err = CssParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "px" => Ok(Unit::Px),
            "pt" => Ok(Unit::Pt),
            "em" => Ok(Unit::Em),
            "%" => Ok(Unit::Percent),
            "vw" => Ok(Unit::Vw),
            "vh" => Ok(Unit::Vh),
            "vmin" => Ok(Unit::Vmin),
            "vmax" => Ok(Unit::Vmax),
            "cm" => Ok(Unit::Cm),
            "mm" => Ok(Unit::Mm),
            "Q" => Ok(Unit::Q),
            "in" => Ok(Unit::In),
            "pc" => Ok(Unit::Pc),
            "fr" => Ok(Unit::Fr),
            _ => Err(CssParseError { css: s.to_string(), message: "unknwon unit".to_string() }),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TextAlign {
    Left,
    Right,
    Center,
}

impl ToString for TextAlign {
    fn to_string(&self) -> String {
        match self {
            TextAlign::Left => "left".to_string(),
            TextAlign::Right => "right".to_string(),
            TextAlign::Center => "center".to_string(),
        }
    }
}

impl FromStr for TextAlign {
    type Err = CssParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "left" => Ok(TextAlign::Left),
            "right" => Ok(TextAlign::Right),
            "center" => Ok(TextAlign::Center),
            _ => Err(CssParseError { css: s.to_string(), message: "invalid or unsupported enum value".to_string() }),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum BoxSizing {
    ContentBox,
    BorderBox,
}

impl ToString for BoxSizing {
    fn to_string(&self) -> String {
        match self {
            BoxSizing::ContentBox => "content-box".to_string(),
            BoxSizing::BorderBox => "border-box".to_string(),
        }
    }
}

impl FromStr for BoxSizing {
    type Err = CssParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "content-box" => Ok(BoxSizing::ContentBox),
            "border-box" => Ok(BoxSizing::BorderBox),
            _ => Err(CssParseError { css: s.to_string(), message: "invalid or unsupported enum value".to_string() }),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Display {
    Block,
    InlineBlock,
    Inline,
    Flex,
    Grid,
    None,
}

impl ToString for Display {
    fn to_string(&self) -> String {
        match self {
            Display::Block => "block".to_string(),
            Display::InlineBlock => "inline-block".to_string(),
            Display::Inline => "inline".to_string(),
            Display::Flex => "flex".to_string(),
            Display::Grid => "grid".to_string(),
            Display::None => "none".to_string(),
        }
    }
}

impl FromStr for Display {
    type Err = CssParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "block" => Ok(Display::Block),
            "inline-block" => Ok(Display::InlineBlock),
            "inline" => Ok(Display::Inline),
            "flex" => Ok(Display::Flex),
            "grid" => Ok(Display::Grid),
            "none" => Ok(Display::None),
            _ => Err(CssParseError { css: s.to_string(), message: "invalid or unsupported enum value".to_string() }),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

impl ToString for FlexDirection {
    fn to_string(&self) -> String {
        match self {
            FlexDirection::Row => "row".to_string(),
            FlexDirection::RowReverse => "row-reverse".to_string(),
            FlexDirection::Column => "column".to_string(),
            FlexDirection::ColumnReverse => "column-reverse".to_string(),
        }
    }
}

impl FromStr for FlexDirection {
    type Err = CssParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "row" => Ok(FlexDirection::Row),
            "row-reverse" => Ok(FlexDirection::RowReverse),
            "column" => Ok(FlexDirection::Column),
            "column-reverse" => Ok(FlexDirection::ColumnReverse),
            _ => Err(CssParseError { css: s.to_string(), message: "invalid or unsupported enum value".to_string() }),
        }
    }
}

/// CSS specificity. Used to resolve conflicts between rules.
/// 
/// ```
/// use yargl::css::Specificity;
/// let a = Specificity { num_id: 1, num_class: 0, num_tag: 1 };
/// let b = Specificity { num_id: 0, num_class: 2, num_tag: 0 };
/// assert!(a > b);
/// let c = Specificity { num_id: 0, num_class: 2, num_tag: 1 };
/// assert!(a > c);
/// assert!(c > b);
/// let d = Specificity { num_id: 0, num_class: 2, num_tag: 1 };
/// assert_eq!(c, d);
/// assert!(b + d > c);
/// ```
#[derive(Clone, PartialEq, Debug)]
pub struct Specificity {
    pub num_id: u32,
    pub num_class: u32,
    pub num_tag: u32,
}

impl PartialOrd for Specificity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.num_id == other.num_id {
            if self.num_class == other.num_class {
                self.num_tag.partial_cmp(&other.num_tag)
            } else {
                self.num_class.partial_cmp(&other.num_class)
            }
        } else {
            self.num_id.partial_cmp(&other.num_id)
        }
    }
}

impl std::ops::Add for Specificity {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Specificity {
            num_id: self.num_id + rhs.num_id,
            num_class: self.num_class + rhs.num_class,
            num_tag: self.num_tag + rhs.num_tag,
        }
    }
}

impl Specificity {
    pub fn new(num_id: u32, num_class: u32, num_tag: u32) -> Specificity {
        Specificity {
            num_id,
            num_class,
            num_tag,
        }
    }

    pub fn add_id(&mut self) {
        self.num_id += 1;
    }
    pub fn add_class(&mut self) {
        self.num_class += 1;
    }
    pub fn add_classes(&mut self, n: u32) {
        self.num_class += n;
    }
    pub fn add_tag(&mut self) {
        self.num_tag += 1;
    }
}

/// A CSS selector.
#[derive(Clone, Debug, PartialEq)]
pub struct Selector {
    pub tag_name: Option<String>,
    pub class_list: Vec<String>,
    pub id: Option<String>,
}

impl Selector {
    /// Creates a new selector with the given tag name, class list, and id.
    pub fn new(tag_name: Option<String>, class_list: Vec<String>, id: Option<String>) -> Selector {
        Selector {
            tag_name,
            class_list,
            id,
        }
    }

    /// Returns a selector with the tag name, all classes, and id (if it exists) of the given node.
    pub fn complete_selector(node: &tl::Node) -> Selector {
        let mut class_list = Vec::new();
        let mut id = None;
        let mut tag_name = None;
        if let Some(tag) = node.as_tag() {
            tag_name = tag.name().try_as_utf8_str().map_or(None, |t| Some(t.to_string()));
            let class_attr = tag.attributes().get("class")
            .unwrap_or(None)
            .map_or(None, |class_attr| class_attr.try_as_utf8_str())
            .unwrap_or("");
            for class in class_attr.split(" ") {
                if !class.is_empty() {
                    class_list.push(class.to_string());
                }
            }
            id = tag.attributes().get("id")
            .unwrap_or(None)
            .map_or(None, |id_attr| id_attr.try_as_utf8_str())
            .map_or(None, |id_attr| Some(id_attr.to_string()));
        }
        Selector::new(tag_name, class_list, id)
    }
    
    /// Returns the specificity of this selector.
    /// 
    /// ```
    /// use yargl::css::{Selector, Specificity};
    /// let a = Selector::new(None, vec!["a".to_string(), "b".to_string()], None);
    /// assert_eq!(a.specificity(), Specificity::new(0, 2, 0));
    /// let b = Selector::new(Some("div".to_string()), vec!["a".to_string(), "b".to_string()], None);
    /// assert_eq!(b.specificity(), Specificity::new(0, 2, 1));
    /// let c = Selector::new(Some("div".to_string()), vec!["a".to_string(), "b".to_string()], Some("c".to_string()));
    /// assert_eq!(c.specificity(), Specificity::new(1, 2, 1));
    /// ```
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
}

impl ToString for Selector {
    /// Returns a string representation of this selector (valid CSS).
    /// 
    /// ```
    /// use yargl::css::Selector;
    /// let a = Selector::new(None, vec!["a".to_string(), "b".to_string()], None);
    /// let a_str = a.to_string();
    /// assert_eq!(a_str, ".a.b");
    /// let b = Selector::new(Some("div".to_string()), vec!["a".to_string(), "b".to_string()], None);
    /// let b_str = b.to_string();
    /// assert_eq!(b_str, "div.a.b");
    /// let c = Selector::new(Some("div".to_string()), vec!["a".to_string(), "b".to_string()], Some("c".to_string()));
    /// let c_str = c.to_string();
    /// assert_eq!(c_str, "div.a.b#c");
    /// let d = Selector::new(None, vec![], None);
    /// let d_str = d.to_string();
    /// assert_eq!(d_str, "");
    /// ```
    fn to_string(&self) -> String {
        let mut selector = String::new();
        if let Some(tag_name) = &self.tag_name {
            selector.push_str(tag_name);
        }
        for class in &self.class_list {
            selector.push('.');
            selector.push_str(class);
        }
        if let Some(id) = &self.id {
            selector.push('#');
            selector.push_str(id);
        }
        selector
    }
}

/// Use this type to retrieve colors from styles.
#[derive(Debug, PartialEq, Clone)]
pub struct CssColor {
    pub sdl_color: sdl2::pixels::Color,
}

impl FromStr for CssColor {
    type Err = CssParseError;
    /// Parses a color from a CSS color value. Only hex colors (no shorthand formats) are supported (yet).
    /// ```
    /// use yargl::css::CssColor;
    /// use std::str::FromStr;
    /// let color = CssColor::from_str("#ff00ff").unwrap();
    /// assert_eq!(color.sdl_color, sdl2::pixels::Color::RGB(255, 0, 255));
    /// let color = CssColor::from_str("#000000").unwrap();
    /// assert_eq!(color.sdl_color, sdl2::pixels::Color::RGB(0, 0, 0));
    /// let color = CssColor::from_str("#ffffff").unwrap();
    /// assert_eq!(color.sdl_color, sdl2::pixels::Color::RGB(255, 255, 255));
    /// let color = CssColor::from_str("#fa017f").unwrap();
    /// assert_eq!(color.sdl_color, sdl2::pixels::Color::RGB(250, 1, 127));
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        if chars.next() == Some('#') {
            let mut color: Vec<u8> = Vec::new();
            for _ in 0..3 {
                let mut hex: String = String::new();
                for _ in 0..2 {
                    match chars.next() {
                        Some(ch) => hex.push(ch),
                        None => return Err(CssParseError { css: s.to_string(), message: "invalid or unsupported enum value".to_string() })
                    }
                }
                match u8::from_str_radix(&hex, 16) {
                    Ok(num) => color.push(num),
                    Err(_) => return Err(CssParseError { css: s.to_string(), message: "invalid or unsupported enum value".to_string() })
                }
            }
            Ok(CssColor { sdl_color: sdl2::pixels::Color::RGB(color[0], color[1], color[2]) })
        } else {
            Err(CssParseError { css: s.to_string(), message: "invalid or unsupported enum value".to_string() })
        }
    }
}

#[derive(Debug)]
/// Holds data of a CSS style rule.
pub struct Style {
    pub selectors: Vec<Selector>,
    properties: HashMap<String, String>,
}

impl Style {
    /// Returns the value of the given property, with a unit if it has one.
    /// For supported units, see [Unit].
    /// ```
    /// use yargl::css::{Style, Unit, parse_css, CssColor};
    /// use std::vec::Vec;
    /// use std::collections::HashMap;
    /// let mut styles = parse_css("div { width: 100px; font-size: 20px; height: 100%; color: #ff0000; }").unwrap();
    /// let mut style = styles.remove(0);
    /// let (width, unit) = style.get_value::<f32>("width");
    /// assert!(width.is_some());
    /// assert!(unit.is_some());
    /// assert_eq!(width.unwrap(), 100.0);
    /// assert_eq!(unit.unwrap(), Unit::Px);
    /// let (font_size, unit) = style.get_value::<f32>("font-size");
    /// assert!(font_size.is_some());
    /// assert!(unit.is_some());
    /// assert_eq!(font_size.unwrap(), 20.0);
    /// assert_eq!(unit.unwrap(), Unit::Px);
    /// let (height, unit) = style.get_value::<f32>("height");
    /// assert!(height.is_some());
    /// assert!(unit.is_some());
    /// assert_eq!(height.unwrap(), 100.0);
    /// assert_eq!(unit.unwrap(), Unit::Percent);
    /// let (margin, unit) = style.get_value::<f32>("margin-top");
    /// assert!(margin.is_none());
    /// assert!(unit.is_none());
    /// let (color, unit) = style.get_value::<CssColor>("color");
    /// assert!(color.is_some());
    /// assert!(unit.is_none());
    /// assert_eq!(color.unwrap().sdl_color, sdl2::pixels::Color::RGB(255, 0, 0));
    /// ```
    pub fn get_value<T>(&self, property: &str) -> (Option<T>, Option<Unit>) where T: std::str::FromStr {
        if let Some(value) = self.properties.get(property) {
            for unit in Unit::iter() {
                let unit_str = unit.to_string();
                if value.ends_with(unit_str.as_str()) {
                    let val = value[0..value.len() - unit_str.len()].parse::<T>().ok();
                    return (val, Some(unit));
                }
            }
            (value.parse::<T>().ok(), None)
        } else {
            (None, None)
        }
    }
    /// Sets the value of the given property, optionally with a unit.
    pub fn set_value<T>(&mut self, property: &str, value: &T, unit: Option<Unit>) where T: std::string::ToString {
        let mut value = value.to_string();
        if let Some(unit) = unit {
            value.push_str(unit.to_string().as_str());
        }
        self.properties.insert(property.to_string(), value.to_string());
    }
}

#[derive(Debug)]
/// A style that was selected for a property on a specific node, based on it's specificity.
pub struct SelectedStyle {
    specificity: Specificity,
    style: Rc<Style>,
}

impl SelectedStyle {
    /// Returns the value of the given property, with a unit if it has one.
    pub fn get_value<T>(&self, property: &str) -> (Option<T>, Option<Unit>) where T: std::str::FromStr {
        self.style.get_value(property)
    }
    /// Returns the specificity based on which the style was selected. This should be the specificity of one
    /// of the matching selectors of the underlying style.
    pub fn specificity(&self) -> &Specificity {
        &self.specificity
    }
}

#[derive(Debug)]
/// A style that was computed for a specific node.
pub struct ComputedStyle {
    selector: Selector,
    pub properties: HashMap<String, SelectedStyle>,
}

impl ComputedStyle {
    pub fn new(selector: Selector) -> ComputedStyle {
        ComputedStyle {
            selector,
            properties: HashMap::new()
        }
    }

    pub fn selector(&self) -> &Selector {
        &self.selector
    }

    pub fn get_value<T>(&self, property: &str) -> (Option<T>, Option<Unit>) where T: std::str::FromStr {
        match self.properties.get(property) {
            Some(property_style) => property_style.get_value(property),
            None => (None, None)
        }
    }

    /// Applies the given style to this computed style, if it has a higher specificity.
    /// 
    /// ```
    /// use yargl::css::{Style, parse_css, Selector, ComputedStyle, Unit, Specificity};
    /// use std::vec::Vec;
    /// use std::collections::HashMap;
    /// let mut styles = parse_css("div.a, #b, .c { width: 100px; font-size: 20px; height: 100%; color: #ff0000; } div.a#d { width: 200px; }").unwrap();
    /// let mut style1 = styles.remove(0);
    /// let mut style2 = styles.remove(0);
    /// let selector = Selector { tag_name: Some("div".to_string()), class_list: vec!["a".to_string()], id: Some("d".to_string()) };
    /// let mut computed_style = ComputedStyle::new(selector);
    /// computed_style.apply_style(style1.clone(), &style1.selectors.get(0).unwrap().specificity());
    /// assert_eq!(computed_style.get_value::<f32>("width"), (Some(100.0), Some(Unit::Px)));
    /// assert_eq!(computed_style.get_value::<f32>("font-size"), (Some(20.0), Some(Unit::Px)));
    /// assert_eq!(computed_style.get_value::<f32>("height"), (Some(100.0), Some(Unit::Percent)));
    /// computed_style.apply_style(style2.clone(), &style2.selectors.get(0).unwrap().specificity());
    /// assert_eq!(computed_style.get_value::<f32>("width"), (Some(200.0), Some(Unit::Px)));
    /// ```
    pub fn apply_style(&mut self, style: Rc<Style>, specificity: &Specificity) {
        for (property, _value) in style.properties.iter() {
            match self.properties.get(property) {
                Some(old_style) => {
                    if specificity > old_style.specificity() {
                        self.properties.insert(property.to_string(), SelectedStyle { 
                            specificity: specificity.clone(),
                            style: style.clone()
                        });
                    }
                },
                None => {
                    self.properties.insert(property.to_string(), SelectedStyle { 
                        specificity: specificity.clone(),
                        style: style.clone()
                    });
                }
            }
        }
    }
}

impl ToString for ComputedStyle {
    /// Formats this computed style rule as a CSS rule.
    fn to_string(&self) -> String {
        let mut result = String::new();
        result.push_str(&self.selector.to_string());
        result.push_str(" {\n");
        for (property, selected_style) in self.properties.iter() {
            let (value, unit) = selected_style.get_value::<String>(property);
            if let Some(str_value) = match unit {
                Some(unit) => value.map(|v| format!("{}{}", v, unit.to_string())),
                None => value
            } {
                result.push_str(&format!("\t{}: {};\n", property, str_value));
            }
        }
        result.push_str("}");
        result
    }
}

/// Parses style rules from css.
/// 
/// - css [&str]: CSS-formatted string
/// 
/// Returns: A vector of reference-counted style rules.
/// 
/// Supports: Comments, Basic selectors (tag, multiple classes, id), Multiple selectors for one style (comma-separated), Units, Hex colors (no shorthand)
// TODO: Pseudo-classes, Pseudo-elements, Attribute selectors, Combinators, Media queries, Keyframes, Animations, Transitions, Variables, Functions, Calc, etc.
pub fn parse_css(css: &str) -> Result<Vec<Rc<Style>>, CssParseError> {
    let mut parser_input = cssparser::ParserInput::new(css);
    let mut parser = cssparser::Parser::new(&mut parser_input);
    let mut sheet: Vec<Rc<Style>> = Vec::new();
    let mut selectors: Vec<Selector> = Vec::new();
    let mut current_selector = Selector::new(None, Vec::new(), None);
    let mut expect_class = false;
    loop {
        match parser.next() {
            Ok(token) => {
                match token {
                    cssparser::Token::Ident(ident) => {
                        if expect_class {
                            current_selector.class_list.push(ident.to_string());
                            expect_class = false;
                        } else {
                            if current_selector.tag_name.is_none() {
                                current_selector.tag_name = Some(ident.to_string());
                            }
                        }
                    },
                    cssparser::Token::Delim(_) => {
                        expect_class = true;
                    },
                    cssparser::Token::IDHash(id) => {
                        if current_selector.id.is_none() {
                            current_selector.id = Some(id.to_string());
                        }
                    },
                    cssparser::Token::Comma => {
                        selectors.push(current_selector.clone());
                        current_selector = Selector::new(None, Vec::new(), None);
                    },
                    cssparser::Token::CurlyBracketBlock => {
                        selectors.push(current_selector.clone());
                        current_selector = Selector::new(None, Vec::new(), None);
                        // Parse Style block
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
                                            cssparser::Token::Comma => {
                                                if in_property_value {
                                                    property_value.push(',');
                                                }
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
                                            cssparser::Token::IDHash(hash) => {
                                                property_value.push('#');
                                                property_value.push_str(&hash.to_string());
                                            },
                                            cssparser::Token::Hash(hash) => {
                                                property_value.push('#');
                                                property_value.push_str(&hash.to_string());
                                            },
                                            cssparser::Token::QuotedString(string) => {
                                                property_value.push_str(string.as_ref());
                                            },
                                            cssparser::Token::WhiteSpace(ws) => {
                                                if in_property_value && !property_value.is_empty() {
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
                                            // This error is only handled so that rust can infer the error type
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
                                sheet.push(Rc::new(style));
                            },
                            Err(parse_error) => {
                                return Err(CssParseError { 
                                    css: "{".to_string(), 
                                    message: format!("Error inside style block: {:?} (in line {}:{})", parse_error.kind, parse_error.location.line, parse_error.location.column)
                                });
                            }
                        }
                    },
                    cssparser::Token::CloseCurlyBracket => {
                        return Err(CssParseError { css: "}".to_string(), message: "Unexpected token".to_string() });
                    },
                    cssparser::Token::BadString(s) => {
                        return Err(CssParseError { css: s.to_string(), message: "Bad string".to_string() });
                    },
                    _ => {}
                }
            },
            Err(_) => break
        }
    }
    Ok(sheet)
}
