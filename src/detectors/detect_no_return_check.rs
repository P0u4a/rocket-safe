use clang::*;
use source::Location;

pub fn detect_no_return_check(tu: &TranslationUnit) -> Vec<String> {
    // TODO replace with hashset?
    let allowed_parents = vec![
        EntityKind::VarDecl,
        EntityKind::IfStmt,
        EntityKind::WhileStmt,
        EntityKind::CStyleCastExpr,
        EntityKind::BinaryOperator,
    ];

    let mut warnings: Vec<String> = vec![];

    let _ = tu.get_entity().visit_children(|child, parent| {
        if child.get_kind() == EntityKind::CallExpr && !allowed_parents.contains(&parent.get_kind())
        {
            let Location {
                line, column, file, ..
            } = child.get_location().unwrap().get_spelling_location();
            warnings.push(format!(
                "return value of {} ignored at line {} column {} in {:?}. If the function does not return anything it should be cast to void.",
                child.get_name().unwrap(),
                line,
                column,
                file.unwrap().get_path().file_name().unwrap(),
            ));
        }
        EntityVisitResult::Recurse
    });

    return warnings;
}
