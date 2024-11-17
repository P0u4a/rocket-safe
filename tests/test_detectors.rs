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

    let expected_warnings = vec![
        "Dynamic memory allocation at line 5 column 25 in \"heap.c\"",
        "Dynamic memory release at line 7 column 3 in \"heap.c\"",
        "Return value of free ignored at line 7 column 3 in \"heap.c\". If the function does not return anything it should be cast to void.",
        "Function factorial called recursively at line 1 column 5 in \"recursion.c\"",
        "Function fibonacci called recursively at line 9 column 5 in \"recursion.c\"",
        "longjmp usage at line 9 column 3 in \"jmp_and_goto.c\"",
        "goto usage at line 16 column 5 in \"jmp_and_goto.c\"",
        "goto usage at line 18 column 5 in \"jmp_and_goto.c\"",
        "setjmp usage at line 27 column 7 in \"jmp_and_goto.c\"",
        "Global variable buf declared at line 4 column 9 in \"jmp_and_goto.c\"",
        "Return value of printf ignored at line 7 column 3 in \"jmp_and_goto.c\". If the function does not return anything it should be cast to void.",
        "Return value of longjmp ignored at line 9 column 3 in \"jmp_and_goto.c\". If the function does not return anything it should be cast to void.",
        "Return value of printf ignored at line 11 column 3 in \"jmp_and_goto.c\". If the function does not return anything it should be cast to void.",
        "Global variable TOP declared at line 1 column 5 in \"global.c\"",
        "While loop at line 4 column 3 is unbounded in \"unbound_loop.c\"",
        "For loop at line 34 column 3 is unbounded in \"unbound_loop.c\"",
        "Double pointer dereference detected at line 2 column 9 in \"pointers.c\"",
        "Double pointer dereference detected at line 2 column 9 in \"pointers.c\"",
        "Double pointer dereference detected at line 11 column 14 in \"pointers.c\"",
        "Return value of is_neg ignored at line 4 column 3 in \"check_return.c\". If the function does not return anything it should be cast to void.",
    ];

    let clang = Clang::new().expect("Failed to initialize clang");
    let index = Index::new(&clang, false, false);

    let all_warnings: Vec<String> = targets
        .into_iter()
        .flat_map(|f| {
            let tu = index.parser(f).parse().expect("Failed to load source file");

            vec![
                detect_recursion::detect_recursion(&tu),
                detect_heap::detect_heap(&tu),
                detect_complex_control_flow::detect_complex_control_flow(&tu),
                detect_globals::detect_globals(&tu),
                detect_unbound_loops::detect_unbound_loops(&tu),
                detect_pointer_deref::detect_pointer_deref(&tu),
                detect_no_return_check::detect_no_return_check(&tu),
            ]
            .into_iter()
            .flatten()
        })
        .collect();

    assert_eq!(all_warnings, expected_warnings);
}
