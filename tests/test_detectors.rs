extern crate rocket_safe;
use clang::*;
use rocket_safe::detectors::*;

#[test]
fn test_detectors() {
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

        let all_warnings: Vec<String> = vec![
            detect_recursion::detect_recursion(&tu),
            detect_heap::detect_heap(&tu),
            detect_complex_control_flow::detect_complex_control_flow(&tu),
            detect_globals::detect_globals(&tu),
            detect_unbound_loops::detect_unbound_loops(&tu),
            detect_pointer_deref::detect_pointer_deref(&tu),
            detect_no_return_check::detect_no_return_check(&tu),
        ]
        .into_iter()
        .flat_map(|v| v)
        .collect();

        all_warnings.into_iter().for_each(|w| {
            println!("{}", w);
        })
    });
}
