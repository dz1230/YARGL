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
    BorderTopWidth,
    BorderBottomWidth,
    BorderLeftWidth,
    BorderRightWidth,
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
    BorderTopLeftRadius,
    BorderTopRightRadius,
    BorderBottomLeftRadius,
    BorderBottomRightRadius,
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
            LayoutValue::BorderTopWidth => 9,
            LayoutValue::BorderBottomWidth => 10,
            LayoutValue::BorderLeftWidth => 11,
            LayoutValue::BorderRightWidth => 12,
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
            LayoutValue::BorderTopLeftRadius => 23,
            LayoutValue::BorderTopRightRadius => 24,
            LayoutValue::BorderBottomLeftRadius => 25,
            LayoutValue::BorderBottomRightRadius => 26,
            LayoutValue::MaxValue => 27,
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
            9 => LayoutValue::BorderTopWidth,
            10 => LayoutValue::BorderBottomWidth,
            11 => LayoutValue::BorderLeftWidth,
            12 => LayoutValue::BorderRightWidth,
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
            23 => LayoutValue::BorderTopLeftRadius,
            24 => LayoutValue::BorderTopRightRadius,
            25 => LayoutValue::BorderBottomLeftRadius,
            26 => LayoutValue::BorderBottomRightRadius,
            27 => LayoutValue::MaxValue,
            _ => panic!("Invalid layout value"),
        }
    }
}

impl FromStr for LayoutValue {
    type Err = LayoutErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "width" => Ok(LayoutValue::Width),
            "height" => Ok(LayoutValue::Height),
            "font-size" => Ok(LayoutValue::FontSize),
            "padding-top" => Ok(LayoutValue::PaddingTop),
            "padding-bottom" => Ok(LayoutValue::PaddingBottom),
            "padding-left" => Ok(LayoutValue::PaddingLeft),
            "padding-right" => Ok(LayoutValue::PaddingRight),
            "border-top-width" => Ok(LayoutValue::BorderTopWidth),
            "border-bottom-width" => Ok(LayoutValue::BorderBottomWidth),
            "border-left-width" => Ok(LayoutValue::BorderLeftWidth),
            "border-right-width" => Ok(LayoutValue::BorderRightWidth),
            "margin-top" => Ok(LayoutValue::MarginTop),
            "margin-bottom" => Ok(LayoutValue::MarginBottom),
            "margin-left" => Ok(LayoutValue::MarginLeft),
            "margin-right" => Ok(LayoutValue::MarginRight),
            "border-top-left-radius" => Ok(LayoutValue::BorderTopLeftRadius),
            "border-top-right-radius" => Ok(LayoutValue::BorderTopRightRadius),
            "border-bottom-left-radius" => Ok(LayoutValue::BorderBottomLeftRadius),
            "border-bottom-right-radius" => Ok(LayoutValue::BorderBottomRightRadius),
            _ => Err(LayoutErr),
        }
    }
}

impl ToString for LayoutValue {
    fn to_string(&self) -> String {
        match self {
            LayoutValue::Width => "width".to_string(),
            LayoutValue::Height => "height".to_string(),
            LayoutValue::FontSize => "font-size".to_string(),
            LayoutValue::PaddingTop => "padding-top".to_string(),
            LayoutValue::PaddingBottom => "padding-bottom".to_string(),
            LayoutValue::PaddingLeft => "padding-left".to_string(),
            LayoutValue::PaddingRight => "padding-right".to_string(),
            LayoutValue::BorderTopWidth => "border-top-width".to_string(),
            LayoutValue::BorderBottomWidth => "border-bottom-width".to_string(),
            LayoutValue::BorderLeftWidth => "border-left-width".to_string(),
            LayoutValue::BorderRightWidth => "border-right-width".to_string(),
            LayoutValue::MarginTop => "margin-top".to_string(),
            LayoutValue::MarginBottom => "margin-bottom".to_string(),
            LayoutValue::MarginLeft => "margin-left".to_string(),
            LayoutValue::MarginRight => "margin-right".to_string(),
            LayoutValue::BorderTopLeftRadius => "border-top-left-radius".to_string(),
            LayoutValue::BorderTopRightRadius => "border-top-right-radius".to_string(),
            LayoutValue::BorderBottomLeftRadius => "border-bottom-left-radius".to_string(),
            LayoutValue::BorderBottomRightRadius => "border-bottom-right-radius".to_string(),
            _ => panic!("Invalid layout value"),
        }
    }
}

pub struct  NodeLayoutInfo {
    values: Vec<Option<i32>>,
}

impl NodeLayoutInfo {
    pub fn new() -> NodeLayoutInfo {
        NodeLayoutInfo {
            values: vec![None; LayoutValue::MaxValue as usize],
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
