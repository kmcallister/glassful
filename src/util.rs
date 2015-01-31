use syntax::ast;
use std::borrow::ToOwned;

pub fn simple_path(p: &ast::Path) -> Option<String> {
    match &p.segments[] {
        [ref single] if single.parameters.is_empty()
            => Some(single.identifier.as_str().to_owned()),
        _ => None,
    }
}

pub fn pat_to_var(p: &ast::Pat) -> Option<String> {
    match p.node {
        ast::PatIdent(ast::BindByValue(ast::MutImmutable), id, None)
            => Some(id.node.as_str().to_owned()),
        _ => None,
    }
}
