
// these arent in an enum because rust doesnt allow const generics with enums yet (on stable releases)

pub const X: usize = 0;
pub const Y: usize = 1;
pub const WIDTH: usize = 2;
pub const HEIGHT: usize = 3;
pub const FONT_SIZE: usize = 4;

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
