use clang::*;
use rocket_safe::detectors::detect_complex_control_flow::detect_complex_control_flow;
use rocket_safe::detectors::detect_globals::detect_globals;
use rocket_safe::detectors::detect_heap::detect_heap;
use rocket_safe::detectors::detect_recursion::detect_recursion;
fn main() {
    let targets = vec![
        "../rocket-safe/tests/code_samples/heap.c",
        "../rocket-safe/tests/code_samples/recursion.c",
        "../rocket-safe/tests/code_samples/jmp_and_goto.c",
        "../rocket-safe/tests/code_samples/global.c",
        "../rocket-safe/tests/code_samples/unbound_loop.c",
        "../rocket-safe/tests/code_samples/pointers.c",
        "../rocket-safe/tests/code_samples/check_return.c",
    ];

    let clang = Clang::new().expect("Failed to initialize clang");
    let index = Index::new(&clang, false, false);

    targets.into_iter().for_each(|f| {
        let tu = index.parser(f).parse().expect("Failed to load source file");
        detect_recursion(&tu);
        detect_heap(&tu);
        detect_complex_control_flow(&tu);
        detect_globals(&tu);
    });
}

/*
For check return value
    TBD

Pointers
    TBD

Unbound Loop
    TBD


*/
