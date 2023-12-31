use std::path::PathBuf;
use std::sync::Arc;

use crate::compiler::lex::{lexable::*, Instruction, Meta, Node, Value};
use crate::token;

#[derive(PartialEq, Eq, Debug)]
pub enum ItemInner {
    Meta(Meta),
    Node(Node),
}

#[derive(PartialEq, Eq, Debug)]
pub struct Item {
    pub item: ItemInner,
    pub file: Arc<PathBuf>,
}

impl<'b> LexableWith<'b, Arc<PathBuf>> for Item {
    fn lex_with(buf: &'b str, file: Arc<PathBuf>) -> LexResult<'b, Self> {
        let (item, buf) = ItemInner::lex(buf)?;
        Ok((Self { item, file }, buf))
    }
}

impl<'b> Lexable<'b> for ItemInner {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        if buf.starts_with('#') {
            let (dir, buf) = Meta::lex(buf)?;
            return Ok((Self::Meta(dir), buf));
        }

        if expect(buf, ".").is_ok() {
            let (label, buf) = token!(buf; '_' |'.')?;
            let buf = ignore_whitespace(buf);
            let buf = expect(buf, ":")?;
            return Ok((Self::Node(Node::Label(label.to_string())), buf));
        }

        let (id, buf) = token!(buf; '_')?;

        let buf = ignore_whitespace_noline(buf);
        if let Ok(buf) = expect(buf, ":") {
            return Ok((Self::Node(Node::Label(id.to_string())), buf));
        }

        if buf.is_empty() {
            return Ok((
                Self::Node(Node::Instruction(Instruction {
                    id: id.to_string(),
                    args: vec![],
                })),
                buf,
            ));
        }

        if let Ok(buf) = expect(buf, "\n") {
            return Ok((
                Self::Node(Node::Instruction(Instruction {
                    id: id.to_string(),
                    args: vec![],
                })),
                buf,
            ));
        }

        let (args, buf) = Vec::<Value>::lex(buf)?;
        Ok((
            Self::Node(Node::Instruction(Instruction {
                id: id.to_string(),
                args,
            })),
            buf,
        ))
    }
}
