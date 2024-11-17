use crate::collectors::collect_functions::collect_functions;
use clang::*;

use super::helpers::get_violation_loc;

pub fn detect_recursion(tu: &TranslationUnit) -> Vec<String> {
    let functions = collect_functions(tu);
    let mut warnings: Vec<String> = vec![];

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
            match get_violation_loc(&recursive_call_loc) {
                Ok(location) => {
                    warnings.push(format!(
                        "Function {} called recursively at line {} column {} in {:?}",
                        func_name, location.line, location.column, location.filename
                    ));
                }
                Err(err) => {
                    warnings.push(format!("Recursive function at unknown location. {}", err));
                }
            };
        }
    }

    return warnings;
}
