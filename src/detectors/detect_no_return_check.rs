use std::collections::HashSet;

use clang::*;

use super::helpers::get_violation_loc;

pub fn detect_no_return_check(tu: &TranslationUnit) -> Vec<String> {
    let allowed_parents = HashSet::from([
        EntityKind::VarDecl,
        EntityKind::IfStmt,
        EntityKind::WhileStmt,
        EntityKind::CStyleCastExpr,
        EntityKind::BinaryOperator,
    ]);

    let mut warnings: Vec<String> = vec![];

    let _ = tu.get_entity().visit_children(|child, parent| {
        if child.get_kind() == EntityKind::CallExpr && !allowed_parents.contains(&parent.get_kind())
        {
            match get_violation_loc(&child.get_location()) {
                Ok(location) => {
                    warnings.push(format!(
                        "Return value of {} ignored at line {} column {} in {:?}. If the function does not return anything it should be cast to void.",
                        child.get_name().unwrap(),
                        location.line,
                        location.column,
                        location.filename,
                    ));
                }
                Err(err) => {
                    warnings.push(format!("Return value of a function ignored at unknown location. {}", err));
                }
            };

        }
        EntityVisitResult::Recurse
    });

    return warnings;
}
