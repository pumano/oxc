use oxc_ast::{AstKind, Comment};

#[derive(Debug)]
pub struct CommentWhitespace<'a> {
    pub start: u32,
    pub end: u32,
    pub comments: Vec<Comment>,
    pub leading_node: Option<AstKind<'a>>,
    pub trailing_node: Option<AstKind<'a>>,
    pub containing_node: Option<AstKind<'a>>,
}

impl<'a> CommentWhitespace<'a> {
    pub fn new(start: u32, end: u32, comments: Vec<Comment>) -> Self {
        Self {
            start,
            end,
            comments,
            leading_node: None,
            trailing_node: None,
            containing_node: None,
        }
    }
}
