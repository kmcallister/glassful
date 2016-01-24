use syntax::ast;

pub fn simple_path(p: &ast::Path) -> Option<String> {
    match &p.segments[..] {
        [ref single] if single.parameters.is_empty()
            => Some(single.identifier.name.to_string()),
        _ => None,
    }
}

pub fn pat_to_var(p: &ast::Pat) -> Option<String> {
    match p.node {
        ast::PatIdent(ast::BindingMode::ByValue(ast::MutImmutable), id, None)
            => Some(id.node.name.to_string()),
        _ => None,
    }
}
