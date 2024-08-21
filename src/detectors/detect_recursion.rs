use crate::collectors::collect_functions::collect_functions;
use clang::*;
use source::Location;

pub fn detect_recursion(tu: &TranslationUnit) {
    let functions = collect_functions(tu);

    for function in functions {
        let func_name = function.get_name().unwrap();
        let mut recursive_call_loc = None;
        let res = function.visit_children(|child, _parent| {
            if child.get_kind() == EntityKind::CallExpr
                && child.get_name().unwrap() == function.get_name().unwrap()
            {
                recursive_call_loc = function.get_location();
                return EntityVisitResult::Break;
            }
            EntityVisitResult::Recurse
        });

        if res {
            let Location {
                line, column, file, ..
            } = recursive_call_loc.unwrap().get_spelling_location();
            println!(
                "Function {} called recursively at line {} column {} in {:?}",
                func_name,
                line,
                column,
                file.unwrap().get_path().file_name().unwrap(),
            );
        }
    }
}
