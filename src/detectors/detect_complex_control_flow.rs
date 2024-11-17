use clang::*;
use source::SourceLocation;

use super::helpers::{get_violation_loc, ViolationLocation};

pub fn detect_complex_control_flow(tu: &TranslationUnit) -> Vec<String> {
    let mut jmps_and_gotos: Vec<(Option<SourceLocation>, String)> = vec![];
    let banned_keywords = vec![
        String::from("goto"),
        String::from("longjmp"),
        String::from("_setjmp"),
    ];

    let target_entities = vec![
        EntityKind::CallExpr,
        EntityKind::GotoStmt,
        EntityKind::IndirectGotoStmt,
    ];

    let _ = tu.get_entity().visit_children(|child, _parent| {
        if target_entities.contains(&child.get_kind())
            && banned_keywords.contains(
                &child
                    .get_display_name()
                    .unwrap_or_else(|| String::from("goto")),
            )
        {
            jmps_and_gotos.push((
                child.get_location(),
                child
                    .get_display_name()
                    .unwrap_or_else(|| String::from("goto")),
            ));
        }
        EntityVisitResult::Recurse
    });

    let mut warnings: Vec<String> = vec![];

    for (use_loc, name) in jmps_and_gotos {
        let ViolationLocation {
            line,
            column,
            filename,
        } = match get_violation_loc(&use_loc) {
            Ok(location) => location,
            Err(err) => {
                warnings.push(format!(
                    "goto/setjmp/longjmp detected at unknown location. {}",
                    err
                ));
                continue;
            }
        };

        match name.as_str() {
            "goto" => warnings.push(format!(
                "goto usage at line {} column {} in {:?}",
                line, column, filename
            )),

            "_setjmp" => warnings.push(format!(
                "setjmp usage at line {} column {} in {:?}",
                line, column, filename
            )),

            "longjmp" => warnings.push(format!(
                "longjmp usage at line {} column {} in {:?}",
                line, column, filename
            )),

            _ => continue,
        }
    }

    return warnings;
}
