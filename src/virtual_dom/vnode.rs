//! This module contains the implementation of abstract virtual node.

use std::fmt;
use std::cmp::PartialEq;
use stdweb::web::{INode, Node, Element};
use html::{ScopeEnv, Component, Renderable};
use super::{VDiff, VTag, VText, VComp};

/// Bind virtual element to a DOM reference.
pub enum VNode<CTX, COMP: Component<CTX>> {
    /// A bind between `VTag` and `Element`.
    VTag(VTag<CTX, COMP>),
    /// A bind between `VText` and `TextNode`.
    VText(VText<CTX, COMP>),
    /// A bind between `VComp` and `Element`.
    VComp(VComp<CTX, COMP>),
    /// A holder for any `Node` (necessary for replacing node).
    VRef(Node),
}


impl<CTX: 'static, COMP: Component<CTX>> VDiff for VNode<CTX, COMP> {
    type Context = CTX;
    type Component = COMP;

    /// Get binded node.
    fn get_node(&self) -> Option<Node> {
        match *self {
            VNode::VTag(ref vtag) => {
                vtag.get_node()
            },
            VNode::VText(ref vtext) => {
                vtext.get_node()
            },
            VNode::VComp(ref vcomp) => {
                vcomp.get_node()
            },
            VNode::VRef(ref node) => {
                Some(node.to_owned())
            },
        }
    }

    /// Remove VNode from parent.
    fn remove(self, parent: &Element) {
        match self {
            VNode::VTag(vtag) => vtag.remove(parent),
            VNode::VText(vtext) => vtext.remove(parent),
            VNode::VComp(vcomp) => vcomp.remove(parent),
            VNode::VRef(node) => {
                parent.remove_child(&node).expect("can't remove node by VRef")
            },
        }
    }

    /// Virtual rendering for the node. It uses parent node and existend children (virtual and DOM)
    /// to check the difference and apply patches to the actual DOM represenatation.
    fn apply(&mut self, parent: &Element, opposite: Option<VNode<Self::Context, Self::Component>>, env: ScopeEnv<Self::Context, Self::Component>) {
        match *self {
            VNode::VTag(ref mut vtag) => {
                vtag.apply(parent, opposite, env);
            }
            VNode::VText(ref mut vtext) => {
                vtext.apply(parent, opposite, env);
            }
            VNode::VComp(ref mut vcomp) => {
                vcomp.apply(parent, opposite, env);
            }
            VNode::VRef(_) => {
                // TODO use it for rendering any tag
                unimplemented!("node can't be rendered now");
            }
        }
    }
}

impl<CTX, COMP: Component<CTX>> From<VText<CTX, COMP>> for VNode<CTX, COMP> {
    fn from(vtext: VText<CTX, COMP>) -> Self {
        VNode::VText(vtext)
    }
}

impl<CTX, COMP: Component<CTX>> From<VTag<CTX, COMP>> for VNode<CTX, COMP> {
    fn from(vtag: VTag<CTX, COMP>) -> Self {
        VNode::VTag(vtag)
    }
}

impl<CTX, COMP: Component<CTX>> From<VComp<CTX, COMP>> for VNode<CTX, COMP> {
    fn from(vcomp: VComp<CTX, COMP>) -> Self {
        VNode::VComp(vcomp)
    }
}

impl<CTX: 'static, COMP: Component<CTX>, T: ToString> From<T> for VNode<CTX, COMP> {
    fn from(value: T) -> Self {
        VNode::VText(VText::new(value.to_string()))
    }
}

impl<'a, CTX, COMP: Component<CTX>> From<&'a Renderable<CTX, COMP>> for VNode<CTX, COMP> {
    fn from(value: &'a Renderable<CTX, COMP>) -> Self {
        value.view()
    }
}

impl<CTX, COMP: Component<CTX>> fmt::Debug for VNode<CTX, COMP> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &VNode::VTag(ref vtag) => vtag.fmt(f),
            &VNode::VText(ref vtext) => vtext.fmt(f),
            &VNode::VComp(_) => "Component<>".fmt(f),
            &VNode::VRef(_) => "NodeReference<>".fmt(f),
        }
    }
}

impl<CTX, COMP: Component<CTX>> PartialEq for VNode<CTX, COMP> {
    fn eq(&self, other: &VNode<CTX, COMP>) -> bool {
        match *self {
            VNode::VTag(ref vtag_a) => {
                match *other {
                    VNode::VTag(ref vtag_b) => {
                        vtag_a == vtag_b
                    },
                    _ => false
                }
            }
            VNode::VText(ref vtext_a) => {
                match *other {
                    VNode::VText(ref vtext_b) => {
                        vtext_a == vtext_b
                    },
                    _ => false
                }
            }
            VNode::VComp(_) => {
                // TODO Implement it
                false
            }
            VNode::VRef(_) => {
                // TODO Implement it
                false
            }
        }
    }
}
