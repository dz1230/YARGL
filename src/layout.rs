
// these arent in an enum because rust doesnt allow const generics with enums yet (on stable releases)

use std::str::FromStr;

pub const X: usize = 0;
pub const Y: usize = 1;
pub const WIDTH: usize = 2;
pub const HEIGHT: usize = 3;
pub const FONT_SIZE: usize = 4;

pub struct LayoutErr;

pub enum LayoutValue {
    X,
    Y,
    Width,
    Height,
    FontSize,
    PaddingTop,
    PaddingBottom,
    PaddingLeft,
    PaddingRight,
    BorderTop,
    BorderBottom,
    BorderLeft,
    BorderRight,
    MarginTop,
    MarginBottom,
    MarginLeft,
    MarginRight,
    ContentWidth,
    ContentHeight,
    MaskedX,
    MaskedY,
    MaskedWidth,
    MaskedHeight,
}

impl FromStr for LayoutValue {
    type Err = LayoutErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(LayoutValue::X),
            "y" => Ok(LayoutValue::Y),
            "width" => Ok(LayoutValue::Width),
            "height" => Ok(LayoutValue::Height),
            "font-size" => Ok(LayoutValue::FontSize),
            "padding-top" => Ok(LayoutValue::PaddingTop),
            "padding-bottom" => Ok(LayoutValue::PaddingBottom),
            "padding-left" => Ok(LayoutValue::PaddingLeft),
            "padding-right" => Ok(LayoutValue::PaddingRight),
            "border-top" => Ok(LayoutValue::BorderTop),
            "border-bottom" => Ok(LayoutValue::BorderBottom),
            "border-left" => Ok(LayoutValue::BorderLeft),
            "border-right" => Ok(LayoutValue::BorderRight),
            "margin-top" => Ok(LayoutValue::MarginTop),
            "margin-bottom" => Ok(LayoutValue::MarginBottom),
            "margin-left" => Ok(LayoutValue::MarginLeft),
            "margin-right" => Ok(LayoutValue::MarginRight),
            "content-width" => Ok(LayoutValue::ContentWidth),
            "content-height" => Ok(LayoutValue::ContentHeight),
            "masked-x" => Ok(LayoutValue::MaskedX),
            "masked-y" => Ok(LayoutValue::MaskedY),
            "masked-width" => Ok(LayoutValue::MaskedWidth),
            "masked-height" => Ok(LayoutValue::MaskedHeight),
            _ => Err(LayoutErr),
        }
    }
}

pub struct  NodeLayoutInfo {
    values: Vec<Option<i32>>,
}

impl NodeLayoutInfo {
    pub fn new() -> NodeLayoutInfo {
        NodeLayoutInfo {
            values: vec![None; 5],
        }
    }

    pub fn is_complete(&self) -> bool {
        self.values.iter().all(|v| v.is_some())
    }

    pub fn get<const V: usize>(&self) -> Option<i32> {
        self.values[V]
    }

    pub fn set<const V: usize>(&mut self, value: Option<i32>) {
        self.values[V] = value;
    }
}
