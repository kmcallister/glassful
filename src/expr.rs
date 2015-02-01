use std::fmt::Writer;
use syntax::ast;
use syntax::parse::ParseSess;
use syntax::codemap::Span;
use syntax::diagnostic::SpanHandler;
use syntax::attr::AttrMetaMethods;

fn get_binop(diag: &SpanHandler, sp: Span, binop: ast::BinOp) -> &'static str {
    match binop.node {
        ast::BiAdd => "+",
        ast::BiSub => "-",
        ast::BiMul => "*",
        ast::BiDiv => "/",
        ast::BiAnd => "&&",
        ast::BiOr => "||",
        ast::BiEq => "==",
        ast::BiLt => "<",
        ast::BiLe => "<=",
        ast::BiNe => "!=",
        ast::BiGe => ">=",
        ast::BiGt => ">",
        _ => {
            diag.span_err(sp, "binary operator not supported");
            ""
        }
    }
}

fn get_unop(diag: &SpanHandler, sp: Span, unop: ast::UnOp) -> &'static str {
    match unop {
        ast::UnNot => "!",
        ast::UnNeg => "-",
        _ => {
            diag.span_err(sp, "unary operator not supported");
            ""
        }
    }
}

pub fn translate(sess: &ParseSess,
                 out: &mut String,
                 expr: &ast::Expr) {
    let diag = &sess.span_diagnostic;

    match expr.node {
        ast::ExprLit(ref lit) => match lit.node {
            ast::LitInt(n, _) => {
                write!(out, "{}", n).unwrap();
            }
            ast::LitFloat(ref f, _) | ast::LitFloatUnsuffixed(ref f) => {
                write!(out, "{}", f).unwrap();
            }
            _ => {
                diag.span_err(expr.span, "can't translate this literal");
            }
        },

        ast::ExprPath(ref p) => match ::util::simple_path(p) {
            Some(name) => {
                write!(out, "{}", name).unwrap();
            }

            _ => {
                diag.span_err(expr.span, "can't translate qualified / parametrized name");
                return;
            }
        },

        ast::ExprBinary(binop, ref lhs, ref rhs) => {
            write!(out, "(").unwrap();
            translate(sess, out, &**lhs);
            write!(out, " {} ", get_binop(diag, expr.span, binop)).unwrap();
            translate(sess, out, &**rhs);
            write!(out, ")").unwrap();
        }

        ast::ExprUnary(unop, ref rhs) => {
            write!(out, "({} ", get_unop(diag, expr.span, unop)).unwrap();
            translate(sess, out, &**rhs);
            write!(out, ")").unwrap();
        }

        ast::ExprIf(ref cond, ref thn, ref els) => {
            write!(out, "if (").unwrap();
            translate(sess, out, &**cond);
            write!(out, ") {{\n").unwrap();
            ::block::translate(sess, out, &**thn, false);
            if let Some(els) = els.as_ref() {
                write!(out, "}}\nelse ").unwrap();
                translate(sess, out, &**els);
            } else {
                write!(out, "}}\n").unwrap();
            }
        }

        ast::ExprAssign(ref lhs, ref rhs) => {
            write!(out, "(").unwrap();
            translate(sess, out, &**lhs);
            write!(out, " = ").unwrap();
            translate(sess, out, &**rhs);
            write!(out, ")").unwrap();
        }

        ast::ExprRet(ref val) => {
            write!(out, "return").unwrap();
            if let Some(val) = val.as_ref() {
                write!(out, " ").unwrap();
                translate(sess, out, &**val);
            }
            write!(out, ";\n").unwrap();
        }

        ast::ExprCall(ref fun, ref args) => {
            translate(sess, out, &**fun);
            write!(out, "(").unwrap();
            for (i, arg) in args.iter().enumerate() {
                if i != 0 {
                    write!(out, ", ").unwrap();
                }
                translate(sess, out, &**arg);
            }
            write!(out, ")").unwrap();
        }

        ast::ExprField(ref lhs, id) => {
            translate(sess, out, &**lhs);
            write!(out, ".{}", id.node.as_str()).unwrap();
        }

        ast::ExprParen(ref inside) => translate(sess, out, &**inside),

        ast::ExprBlock(ref inside) => {
            write!(out, "{{\n").unwrap();
            ::block::translate(sess, out, &**inside, false);
            write!(out, "}}\n").unwrap();
        }

        ast::ExprMac(_) => {
            diag.span_bug(expr.span, "macros should be gone by now");
        }

        _ => {
            diag.span_err(expr.span, "can't translate this sort of expression");
        }
    }
}
