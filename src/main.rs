use clang::*;
use rocket_safe::detectors::detect_heap::detect_heap;
use rocket_safe::detectors::detect_recursion::detect_recursion;
fn main() {
    let targets = vec![
        "/home/pouya/rocket-safe/tests/code_samples/heap.c",
        "/home/pouya/rocket-safe/tests/code_samples/recursion.c",
    ];

    let clang = Clang::new().expect("Failed to initialize clang");
    let index = Index::new(&clang, false, false);

    targets.into_iter().for_each(|f| {
        let tu = index.parser(f).parse().expect("Failed to load source file");
        detect_recursion(&tu);
        detect_heap(&tu);
    });
}

/*
IDEA

For jmp and goto
    Search AST recursively for those symbols

Global
    Check the lexical parent of each immediate tu children, if any are global it should be obvious

For check return value
    TBD

Pointers
    TBD

Unbound Loop
    TBD


*/
