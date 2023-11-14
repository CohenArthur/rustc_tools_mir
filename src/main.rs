#![feature(rustc_private)]

extern crate rustc_lint;
extern crate rustc_middle;
extern crate rustc_session;

use rustc_lint::LateLintPass;
use rustc_middle::ty::TyCtxt;

use rustc_session::declare_lint_pass;
use rustc_session::declare_tool_lint;

use rustc_tools::with_lints;

declare_tool_lint! {
    pub lint::DISPLAY_BORROWCK,
    Warn,
    "display borrowchecking information",
    report_in_external_macro: false
}

declare_lint_pass! {DummyLint => [DISPLAY_BORROWCK]}

impl LateLintPass<'_> for DummyLint {}

fn dump_mir_information(tcx: TyCtxt) {
    for key in dbg!(tcx.mir_keys(())) {
        dbg!(tcx.optimized_mir(*key));
    }
}

fn main() -> Result<(), ()> {
    let args: Vec<String> = std::env::args().collect();

    with_lints(&args, vec![], |store| {
        store.register_late_pass(|ty_ctx| {
            dump_mir_information(ty_ctx);
            Box::new(DummyLint)
        })
    })
    .map_err(|_| ())
}
