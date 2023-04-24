use tl::VDom;

pub struct Dom<'a> {
    pub vdom: VDom<'a>,
}

impl Dom<'_> {
    pub fn new(html: &str) -> Dom {
        Dom {
            vdom: tl::parse(html, tl::ParserOptions::default()).unwrap(),
        }
    }
}