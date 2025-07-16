use crate::renderer::{
    dom::node::{Node, Window},
    html::token::HtmlTokenizer,
};
use alloc::{rc::Rc, vec::Vec};
use core::cell::RefCell;

#[derive(Debug, Clone)]
enum InsertionMode {
    Initial,
}

#[derive(Debug, Clone)]
pub struct HtmlParser {
    window: Rc<RefCell<Window>>,
    node: InsertionMode,
    original_insertion_mode: InsertionMode,
    stack_of_open_elements: Vec<Rc<RefCell<Node>>>,
    t: HtmlTokenizer,
}

impl HtmlParser {
    pub fn new(t: HtmlTokenizer) -> Self {
        Self {
            window: Rc::new(RefCell::new(Window::new())),
            node: InsertionMode::Initial,
            original_insertion_mode: InsertionMode::Initial,
            stack_of_open_elements: Vec::new(),
            t,
        }
    }
}
