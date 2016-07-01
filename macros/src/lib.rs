#![crate_type="dylib"]
#![feature(plugin_registrar, slice_patterns)]
#![feature(rustc_private, core)]
#![deny(warnings)]
#![allow(unused_features)]

extern crate syntax;
extern crate glassful;
extern crate rustc_plugin;

use syntax::ast;
use syntax::parse::token;
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacEager, MacResult, DummyResult};
use syntax::ext::build::AstBuilder;
use rustc_plugin::Registry;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("glassful", expand);
}

fn expand(cx: &mut ExtCtxt, outer_span: Span, toks: &[ast::TokenTree])
    -> Box<MacResult + 'static>
{
    let inner_span = match toks {
        [] => {
            cx.span_err(outer_span, "empty invocation");
            return DummyResult::expr(outer_span);
        }
        [ref first, ..] => {
            let first = first.get_span();
            let last = toks.iter().rev().next().unwrap().get_span();
            if first.expn_id != last.expn_id {
                cx.struct_span_err(first, "invocation is split between expansion contexts?").span_note(last, "last token is here");
                return DummyResult::expr(outer_span);
            }

            Span {
                lo: first.lo,
                hi: last.hi,
                expn_id: first.expn_id,
            }
        }
    };

    let src = match cx.codemap().span_to_snippet(inner_span) {
        Err(e) => {
            cx.span_err(inner_span, &format!("can't extract source snippet: {:?}", e)[..]);
            return DummyResult::expr(inner_span);
        }
        Ok(src) => src,
    };

    match glassful::try_translate(src) {
        None => {
            cx.span_err(outer_span, "translation failed");
            DummyResult::expr(outer_span)
        }
        Some(res) => {
            let interned = token::intern_and_get_ident(&res[..]);
            MacEager::expr(cx.expr_str(inner_span, interned))
        }
    }
}
