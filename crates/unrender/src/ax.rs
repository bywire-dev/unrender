//! The accessibility node model.
//!
//! Roles deliberately borrow ARIA vocabulary (`table`, `listitem`, `tree`,
//! `statusbar`, `textbox`...). An LLM already has strong priors about what a
//! `row` inside a `table` means and what you can do to a `button`; reusing
//! those names transfers that prior for free, where an invented taxonomy would
//! have to be explained in every prompt.

use crate::rects::Rect;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Node {
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    pub rect: [u16; 4],
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub states: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(role: &str, rect: Rect) -> Node {
        Node {
            role: role.to_string(),
            name: None,
            value: None,
            rect: [rect.x, rect.y, rect.w, rect.h],
            states: Vec::new(),
            children: Vec::new(),
        }
    }
    pub fn named(mut self, name: Option<String>) -> Node {
        self.name = name.filter(|s| !s.is_empty());
        self
    }
    pub fn valued(mut self, v: impl Into<String>) -> Node {
        let v = v.into();
        if !v.is_empty() {
            self.value = Some(v);
        }
        self
    }
    pub fn state(mut self, s: &str) -> Node {
        self.states.push(s.to_string());
        self
    }
    pub fn rect_of(&self) -> Rect {
        Rect {
            x: self.rect[0],
            y: self.rect[1],
            w: self.rect[2],
            h: self.rect[3],
        }
    }
    pub fn count(&self) -> usize {
        1 + self.children.iter().map(|c| c.count()).sum::<usize>()
    }
    /// Depth-first flatten, used by the scorer.
    pub fn flatten<'a>(&'a self, out: &mut Vec<&'a Node>) {
        out.push(self);
        for c in &self.children {
            c.flatten(out);
        }
    }
}
