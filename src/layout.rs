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
    ContentX,
    ContentY,
    ContentWidth,
    ContentHeight,
    ContentLineWidth,
    ContentLineHeight,
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
            LayoutValue::ContentX => 17,
            LayoutValue::ContentY => 18,
            LayoutValue::ContentWidth => 19,
            LayoutValue::ContentHeight => 20,
            LayoutValue::ContentLineWidth => 21,
            LayoutValue::ContentLineHeight => 22,
            LayoutValue::MaskedX => 23,
            LayoutValue::MaskedY => 24,
            LayoutValue::MaskedWidth => 25,
            LayoutValue::MaskedHeight => 26,
            LayoutValue::BorderTopLeftRadius => 27,
            LayoutValue::BorderTopRightRadius => 28,
            LayoutValue::BorderBottomLeftRadius => 29,
            LayoutValue::BorderBottomRightRadius => 30,
            LayoutValue::MaxValue => 31
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
            17 => LayoutValue::ContentX,
            18 => LayoutValue::ContentY,
            19 => LayoutValue::ContentWidth,
            20 => LayoutValue::ContentHeight,
            21 => LayoutValue::ContentLineWidth,
            22 => LayoutValue::ContentLineHeight,
            23 => LayoutValue::MaskedX,
            24 => LayoutValue::MaskedY,
            25 => LayoutValue::MaskedWidth,
            26 => LayoutValue::MaskedHeight,
            27 => LayoutValue::BorderTopLeftRadius,
            28 => LayoutValue::BorderTopRightRadius,
            29 => LayoutValue::BorderBottomLeftRadius,
            30 => LayoutValue::BorderBottomRightRadius,
            31 => LayoutValue::MaxValue,
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

#[derive(Debug)]
pub struct  NodeLayoutInfo {
    values: Vec<Option<i32>>,
}

impl NodeLayoutInfo {
    pub fn new() -> NodeLayoutInfo {
        NodeLayoutInfo {
            values: vec![None; LayoutValue::MaxValue as usize],
        }
    }

    pub fn is_set<const V: usize>(&self) -> bool {
        self.values[V].is_some()
    }

    // TODO somehow this ends up in correct layout, but reversed. figure out why (current fix is to flip it afterwards)

    /// Updates content flow for an inline child. Returns the child's position. Call in reverse order of children.
    pub fn reverse_flow_inline(&mut self, child_width: i32, child_height: i32) -> (i32, i32) {
        let mut cur_content_x = -self.values[LayoutValue::ContentX as usize].unwrap_or(0);
        let mut child_x = cur_content_x;
        let mut child_y = -self.values[LayoutValue::ContentY as usize].unwrap_or(0);
        
        // possibly break child onto new line
        if let Some(width) = self.values[LayoutValue::Width as usize] {
            if cur_content_x + child_width > width {
                self.reverse_break_line();
                cur_content_x = -self.values[LayoutValue::ContentX as usize].unwrap_or(0);
                child_x = cur_content_x;
                child_y = -self.values[LayoutValue::ContentY as usize].unwrap_or(0);
            }
        }
        
        let cur_content_line_width = self.values[LayoutValue::ContentLineWidth as usize].unwrap_or(0);
        let cur_content_line_height = self.values[LayoutValue::ContentLineHeight as usize].unwrap_or(0);

        self.set::<{LayoutValue::ContentX as usize}>(Some(cur_content_x - child_width));
        self.set::<{LayoutValue::ContentLineWidth as usize}>(Some(cur_content_line_width + child_width));
        self.set::<{LayoutValue::ContentLineHeight as usize}>(Some(cur_content_line_height.max(child_height)));

        (child_x, child_y)
    }

    /// Updates content flow for a block child. Returns the child's position. Call in reverse order of children.
    pub fn reverse_flow_block(&mut self, child_width: i32, child_height: i32) -> (i32, i32) {
        println!("reverse_flow_block: child_width: {}, child_height: {}", child_width, child_height);

        self.reverse_break_line();
        let child_x = -self.values[LayoutValue::ContentX as usize].unwrap_or(0);
        let child_y = -self.values[LayoutValue::ContentY as usize].unwrap_or(0);
        self.set::<{LayoutValue::ContentLineWidth as usize}>(Some(child_width));
        self.set::<{LayoutValue::ContentLineHeight as usize}>(Some(child_height));
        self.reverse_break_line();
        return (child_x, child_y);
    }
    
    // Breaks the line, updates content width and content height based on line width and line height, updates content y, resets content x, line width and line height
    pub fn reverse_break_line(&mut self) {
        let line_width = self.values[LayoutValue::ContentLineWidth as usize].unwrap_or(0);
        let line_height = self.values[LayoutValue::ContentLineHeight as usize].unwrap_or(0);
        let content_width = self.values[LayoutValue::ContentWidth as usize].unwrap_or(0);
        let content_height = self.values[LayoutValue::ContentHeight as usize].unwrap_or(0);

        println!("line_width: {}, line_height: {}, content_width: {}, content_height: {}", line_width, line_height, content_width, content_height);

        self.set::<{LayoutValue::ContentWidth as usize}>(Some(content_width.max(line_width)));
        self.set::<{LayoutValue::ContentHeight as usize}>(Some(content_height + line_height));

        self.set::<{LayoutValue::ContentY as usize}>(Some(-content_height - line_height));
        self.set::<{LayoutValue::ContentX as usize}>(Some(0));
        self.set::<{LayoutValue::ContentLineWidth as usize}>(Some(0));
        self.set::<{LayoutValue::ContentLineHeight as usize}>(Some(0));
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
