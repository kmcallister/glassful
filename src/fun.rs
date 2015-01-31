use std::fmt::Writer;
use syntax::ast;
use syntax::parse::ParseSess;
use syntax::attr::AttrMetaMethods;

pub fn translate(sess: &ParseSess,
                 out: &mut String,
                 name: ast::Ident,
                 inputs: &[ast::Arg],
                 output: Option<&ast::Ty>,
                 block: &ast::Block) {
    let diag = &sess.span_diagnostic;

    match output {
        None => write!(out, "void").unwrap(),
        Some(ty) => ::ty::translate(sess, out, ty),
    }

    write!(out, " {}(", name.as_str()).unwrap();
    for (i, &ast::Arg { ref ty, ref pat, ..}) in inputs.iter().enumerate() {
        if i != 0 {
            write!(out, ", ").unwrap();
        }
        ::ty::translate(sess, out, &**ty);
        match ::util::pat_to_var(&**pat) {
            Some(v) => write!(out, " {}", v).unwrap(),
            _ => {
                diag.span_err(pat.span, "can't translate this sort of pattern");
            }
        }
    }
    write!(out, ") {{\n").unwrap();

    ::block::translate(sess, out, block, true);

    write!(out, "}}\n").unwrap();
}
