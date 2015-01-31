use std::fmt::Writer;
use syntax::ast;
use syntax::parse::ParseSess;
use syntax::attr::AttrMetaMethods;

pub fn translate(sess: &ParseSess,
                 out: &mut String,
                 ty: &ast::Ty) {
    let diag = &sess.span_diagnostic;

    match ty.node {
        ast::TyTup(ref t) if t.len() == 0 => {
            write!(out, "void").unwrap();
        }

        ast::TyPath(ref p, _) => match ::util::simple_path(p) {
            None => {
                diag.span_err(ty.span, "can't translate qualified / parametrized name");
            }
            Some(name) => {
                let name = match &name[] {
                    "f32" => "float",
                    name => name,
                };
                write!(out, "{}", name).unwrap();
            }
        },

        _ => {
            diag.span_err(ty.span, "can't translate this sort of type");
        }
    }
}
