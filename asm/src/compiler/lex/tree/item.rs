use crate::compiler::lex::{lexable::*, Directive, Instruction, Node, Value};

#[derive(PartialEq, Eq, Debug)]
pub enum Item<'i> {
    Directive(Directive<'i>),
    Node(Node<'i>),
}

impl<'b> Lexable<'b> for Item<'b> {
    fn lex(buf: &'b str) -> LexResult<'b, Self> {
        if buf.starts_with("#") {
            let (dir, buf) = Directive::lex(buf)?;
            return Ok((Self::Directive(dir), buf));
        }

        if let Ok(_) = expect(buf, ".") {
            let (label, buf) = collect_while(buf, |c| c.is_alphanumeric() || c == '_' || c == '.')?;
            let buf = ignore_whitespace(buf);
            let buf = expect(buf, ":")?;
            return Ok((Self::Node(Node::Label(label)), buf));
        }

        let (id, buf) = collect_while(buf, |c| c.is_alphanumeric() || c == '_')?;
        let buf = ignore_whitespace(buf);
        if let Ok(buf) = expect(buf, ":") {
            return Ok((Self::Node(Node::Label(id)), buf));
        }

        let (args, buf) = Vec::<Value>::lex(buf)?;
        Ok((Self::Node(Node::Instruction(Instruction { id, args })), buf))
    }
}
