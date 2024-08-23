use clang::{Clang, Index};
use rocket_safe::detectors::detect_complex_control_flow::detect_complex_control_flow;
use rocket_safe::detectors::detect_globals::detect_globals;
use rocket_safe::detectors::detect_heap::detect_heap;
use rocket_safe::detectors::detect_no_return_check::detect_no_return_check;
use rocket_safe::detectors::detect_pointer_deref::detect_pointer_deref;
use rocket_safe::detectors::detect_recursion::detect_recursion;
use rocket_safe::detectors::detect_unbound_loops::detect_unbound_loops;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 || !args[1].ends_with(".c") {
        panic!("Please input a C file to analyse. For example: cargo run -- path/to/main.c ");
    }

    let clang = Clang::new().expect("Failed to initialize clang");
    let index = Index::new(&clang, false, false);

    let tu = index
        .parser(&args[1])
        .parse()
        .expect("Failed to load source file");

    detect_recursion(&tu);
    detect_heap(&tu);
    detect_complex_control_flow(&tu);
    detect_globals(&tu);
    detect_unbound_loops(&tu);
    detect_no_return_check(&tu);
    detect_pointer_deref(&tu);
}
