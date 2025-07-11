use alloc::rc::Rc;
use alloc::rc::Weak;
use alloc::string::String;
use core::cell::RefCell;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeKind {
    Document,
    Element(String),
    Text(String),
}

#[derive(Debug)]
pub struct Window {
    // Windowの実装は今後追加
}

pub struct Node {
    pub kind: NodeKind,
    window: Weak<RefCell<Window>>,
    parent: Weak<RefCell<Node>>,
    first_child: Option<Rc<RefCell<Node>>>,
    last_child: Weak<RefCell<Node>>,
    previous_sibling: Weak<RefCell<Node>>,
    next_sibling: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(kind: NodeKind) -> Self {
        Self {
            kind,
            window: Weak::new(),
            parent: Weak::new(),
            first_child: None,
            last_child: Weak::new(),
            previous_sibling: Weak::new(),
            next_sibling: None,
        }
    }

    // Getters
    pub fn parent(&self) -> &Weak<RefCell<Node>> {
        &self.parent
    }

    pub fn first_child(&self) -> &Option<Rc<RefCell<Node>>> {
        &self.first_child
    }

    pub fn last_child(&self) -> &Weak<RefCell<Node>> {
        &self.last_child
    }

    pub fn previous_sibling(&self) -> &Weak<RefCell<Node>> {
        &self.previous_sibling
    }

    pub fn next_sibling(&self) -> &Option<Rc<RefCell<Node>>> {
        &self.next_sibling
    }

    // Setters
    pub fn set_parent(&mut self, parent: Weak<RefCell<Node>>) {
        self.parent = parent;
    }

    pub fn set_first_child(&mut self, first_child: Option<Rc<RefCell<Node>>>) {
        self.first_child = first_child;
    }

    pub fn set_last_child(&mut self, last_child: Weak<RefCell<Node>>) {
        self.last_child = last_child;
    }

    pub fn set_previous_sibling(&mut self, previous_sibling: Weak<RefCell<Node>>) {
        self.previous_sibling = previous_sibling;
    }

    pub fn set_next_sibling(&mut self, next_sibling: Option<Rc<RefCell<Node>>>) {
        self.next_sibling = next_sibling;
    }
}
