#![feature(rustc_private, core, std_misc)]
#![deny(warnings)]

extern crate syntax;

use std::borrow::ToOwned;
use std::thread::Thread;
use syntax::parse;
use syntax::ext::expand;
use syntax::attr::AttrMetaMethods;

mod item;
mod var;
mod util;
mod ty;
mod fun;
mod expr;
mod block;

const NAME: &'static str = "<glassful shader>";

/// Translate a glassful program to GLSL, or panic.
pub fn translate(source: String) -> String {
    // parse
    let sess = parse::new_parse_sess();
    let diag = &sess.span_diagnostic;
    let krate = parse::parse_crate_from_source_str(
        NAME.to_owned(), source, vec![], &sess);

    diag.handler.abort_if_errors();

    // expand macros
    let ecfg = expand::ExpansionConfig::default(NAME.to_owned());
    let krate = expand::expand_crate(&sess, ecfg, vec![], vec![], krate);

    // process attributes
    let mut glsl_version = None;
    for attr in krate.attrs.iter() {
        if attr.check_name("version") {
            if let Some(val) = attr.value_str() {
                if glsl_version.is_some() {
                    diag.span_err(attr.span, "version given twice");
                }
                glsl_version = Some((*val).to_string());
            } else {
                diag.span_err(attr.span, "version not given");
            }
        } else {
            diag.span_err(attr.span, "unknown attribute");
        }
    }

    diag.handler.abort_if_errors();

    // translate!

    let mut out = match glsl_version {
        Some(v) => format!("#version {}\n\n", v),
        None => "".to_owned(),
    };

    for item in krate.module.items.iter() {
        item::translate(&sess, &mut out, &**item);
    }

    diag.handler.abort_if_errors();

    out
}

/// Translate a glassful program to GLSL, or return `None'.
///
/// Because the `libsyntax` parser uses `panic!` internally,
/// this spawns a new thread for the duration of the call.
pub fn try_translate(source: String) -> Option<String> {
    Option::Some(
        Thread::scoped(move || {translate(source)})
            .join())
}
