use std::str::FromStr;


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
    MaxValue
}

impl Into<usize> for LayoutValue {
    fn into(self) -> usize {
        match self {
            LayoutValue::X => 0,
            LayoutValue::Y => 1,
            LayoutValue::Width => 2,
            LayoutValue::Height => 3,
            LayoutValue::FontSize => 4,
            LayoutValue::PaddingTop => 5,
            LayoutValue::PaddingBottom => 6,
            LayoutValue::PaddingLeft => 7,
            LayoutValue::PaddingRight => 8,
            LayoutValue::BorderTop => 9,
            LayoutValue::BorderBottom => 10,
            LayoutValue::BorderLeft => 11,
            LayoutValue::BorderRight => 12,
            LayoutValue::MarginTop => 13,
            LayoutValue::MarginBottom => 14,
            LayoutValue::MarginLeft => 15,
            LayoutValue::MarginRight => 16,
            LayoutValue::ContentWidth => 17,
            LayoutValue::ContentHeight => 18,
            LayoutValue::MaskedX => 19,
            LayoutValue::MaskedY => 20,
            LayoutValue::MaskedWidth => 21,
            LayoutValue::MaskedHeight => 22,
            LayoutValue::MaxValue => 23,
        }
    }
}

impl From<usize> for LayoutValue {
    fn from(value: usize) -> Self {
        match value {
            0 => LayoutValue::X,
            1 => LayoutValue::Y,
            2 => LayoutValue::Width,
            3 => LayoutValue::Height,
            4 => LayoutValue::FontSize,
            5 => LayoutValue::PaddingTop,
            6 => LayoutValue::PaddingBottom,
            7 => LayoutValue::PaddingLeft,
            8 => LayoutValue::PaddingRight,
            9 => LayoutValue::BorderTop,
            10 => LayoutValue::BorderBottom,
            11 => LayoutValue::BorderLeft,
            12 => LayoutValue::BorderRight,
            13 => LayoutValue::MarginTop,
            14 => LayoutValue::MarginBottom,
            15 => LayoutValue::MarginLeft,
            16 => LayoutValue::MarginRight,
            17 => LayoutValue::ContentWidth,
            18 => LayoutValue::ContentHeight,
            19 => LayoutValue::MaskedX,
            20 => LayoutValue::MaskedY,
            21 => LayoutValue::MaskedWidth,
            22 => LayoutValue::MaskedHeight,
            23 => LayoutValue::MaxValue,
            _ => panic!("Invalid layout value"),
        }
    }
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
