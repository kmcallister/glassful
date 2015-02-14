use std::fmt::Writer;
use syntax::ast;
use syntax::parse::ParseSess;
use syntax::attr::AttrMetaMethods;

pub fn translate(sess: &ParseSess,
                 out: &mut String,
                 attrs: &[ast::Attribute],
                 ident: ast::Ident,
                 ty: &ast::Ty,
                 mut init: Option<&ast::Expr>) {
    let diag = &sess.span_diagnostic;

    for attr in attrs.iter() {
        let name = &*attr.name();
        match name {
            // many others: https://www.opengl.org/wiki/Type_Qualifier_%28GLSL%29
            "varying" | "attribute" | "uniform" => {
                write!(out, "{} ", name).unwrap();
            }
            _ => diag.span_err(attr.span, "unknown variable attribute"),
        }
    }

    // The special ident 'UNINIT' means no initializer.
    // Rust's syntax does not allow this otherwise on statics.
    if let Some(i) = init {
        if let ast::ExprPath(ref p) = i.node {
            if let Some(s) = ::util::simple_path(p) {
                if &s[] == "UNINIT" {
                    init = None;
                }
            }
        }
    }

    ::ty::translate(sess, out, ty);
    write!(out, " {}", ident.as_str()).unwrap();
    if let Some(init) = init {
        write!(out, " = ").unwrap();
        ::expr::translate(sess, out, init);
    }

    write!(out, ";\n").unwrap();
}
