use alloc::string::String;
use alloc::vec::Vec;

use crate::renderer::html::attribute::Attribute;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlTokenizer {
    state: State,
    pos: usize,
    reconsume: bool,
    latest_token: Option<HtmlToken>,
    input: Vec<char>,
    buf: String,
}

impl HtmlTokenizer {
    fn is_eof(&self) -> bool {
        self.pos > self.input.len()
    }

    fn consume_next_input(&mut self) -> char {
        let c = self.input[self.pos];
        self.pos += 1;
        c
    }

    fn create_tag(&mut self, start_tag_token: bool) {
        if start_tag_token {
            self.latest_token = Some(HtmlToken::StartTag {
                tag: String::new(),
                self_closing: false,
                attributes: Vec::new(),
            });
        } else {
            self.latest_token = Some(HtmlToken::EndTag { tag: String::new() });
        }
    }
}

impl Iterator for HtmlTokenizer {
    type Item = HtmlToken;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.input.len() {
            return None;
        }

        loop {
            let c = self.consume_next_input();

            match self.state {
                State::Data => {
                    if c == '<' {
                        self.state = State::TagOpen;
                        continue;
                    }
                    if self.is_eof() {
                        return Some(HtmlToken::EOF);
                    }
                    return Some(HtmlToken::Char(c));
                }
                State::TagOpen => {
                    if c == '/' {
                        self.state = State::EndTagOpen;
                        continue;
                    }
                    if c.is_ascii_alphabetic() {
                        self.reconsume = true;
                        self.state = State::TagName;
                        self.create_tag(true);
                        continue;
                    }
                    if self.is_eof() {
                        return Some(HtmlToken::EOF);
                    }
                    self.reconsume = true;
                    self.state = State::Data;
                }
                State::EndTagOpen => todo!(),
                State::TagName => todo!(),
                State::BeforeAttributeName => todo!(),
                State::AttributeName => todo!(),
                State::AfterAttributeName => todo!(),
                State::BeforeAttributeValue => todo!(),
                State::AttributeValueDoubleQuoted => todo!(),
                State::AttributeValueSingleQuoted => todo!(),
                State::AttributeValueUnquoted => todo!(),
                State::AfterAttributeValueQuoted => todo!(),
                State::SelfClosingStartTag => todo!(),
                State::ScriptData => todo!(),
                State::ScriptDataLessThanSign => todo!(),
                State::ScriptDataEndTagOpen => todo!(),
                State::ScriptDataEndTagName => todo!(),
                State::TemporaryBuffer => todo!(),
            }
        }
    }
}

impl HtmlTokenizer {
    pub fn new(html: String) -> Self {
        Self {
            state: State::Data,
            pos: 0,
            reconsume: false,
            latest_token: None,
            input: html.chars().collect(),
            buf: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HtmlToken {
    StartTag {
        tag: String,
        self_closing: bool,
        attributes: Vec<Attribute>,
    },
    EndTag {
        tag: String,
    },
    Char(char),
    EOF,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    Data,
    TagOpen,
    EndTagOpen,
    TagName,
    BeforeAttributeName,
    AttributeName,
    AfterAttributeName,
    BeforeAttributeValue,
    AttributeValueDoubleQuoted,
    AttributeValueSingleQuoted,
    AttributeValueUnquoted,
    AfterAttributeValueQuoted,
    SelfClosingStartTag,
    ScriptData,
    ScriptDataLessThanSign,
    ScriptDataEndTagOpen,
    ScriptDataEndTagName,
    TemporaryBuffer,
}
