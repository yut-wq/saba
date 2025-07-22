use crate::renderer::{
    dom::node::{Node, Window},
    html::token::{HtmlToken, HtmlTokenizer},
};
use alloc::{rc::Rc, vec::Vec};
use core::cell::RefCell;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InsertionMode {
    Initial,
    BeforeHtml,
    BeforeHead,
    InHead,
    AfterHead,
    InBody,
    Text,
    AfterBody,
    AfterAfterBody,
}

#[derive(Debug, Clone)]
pub struct HtmlParser {
    window: Rc<RefCell<Window>>,
    mode: InsertionMode,
    original_insertion_mode: InsertionMode,
    stack_of_open_elements: Vec<Rc<RefCell<Node>>>,
    t: HtmlTokenizer,
}

impl HtmlParser {
    pub fn new(t: HtmlTokenizer) -> Self {
        Self {
            window: Rc::new(RefCell::new(Window::new())),
            mode: InsertionMode::Initial,
            original_insertion_mode: InsertionMode::Initial,
            stack_of_open_elements: Vec::new(),
            t,
        }
    }

    pub fn construct_tree(&mut self) -> Rc<RefCell<Window>> {
        let mut token = self.t.next();

        while token.is_some() {
            match self.mode {
                InsertionMode::Initial => {
                    // DOCTYPEをサポートしない
                    if let Some(HtmlToken::Char(_)) = token {
                        token = self.t.next();
                        continue;
                    }
                    
                    self.mode = InsertionMode::BeforeHtml;
                    continue;
                }
                InsertionMode::BeforeHtml => {}
                InsertionMode::BeforeHead => {}
                InsertionMode::InHead => {}
                InsertionMode::AfterHead => {}
                InsertionMode::InBody => {}
                InsertionMode::Text => {}
                InsertionMode::AfterBody => {}
                InsertionMode::AfterAfterBody => {}
            }
        }
        
        self.window.clone()
    }
}
