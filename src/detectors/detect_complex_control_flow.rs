use clang::*;
use source::Location;

pub fn detect_complex_control_flow(tu: &TranslationUnit) -> Vec<String> {
    let mut jmps_and_gotos: Vec<(Location, String)> = vec![];
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
            && banned_keywords.contains(&child.get_display_name().unwrap_or(String::from("goto")))
        {
            jmps_and_gotos.push((
                child.get_location().unwrap().get_spelling_location(),
                child.get_display_name().unwrap_or(String::from("goto")),
            ));
        }
        EntityVisitResult::Recurse
    });

    let mut warnings: Vec<String> = vec![];

    for (use_loc, name) in jmps_and_gotos {
        let Location {
            line, column, file, ..
        } = use_loc;

        match name.as_str() {
            "goto" => warnings.push(format!(
                "goto usage at line {} column {} in {:?}",
                line,
                column,
                file.unwrap().get_path().file_name().unwrap()
            )),

            "_setjmp" => warnings.push(format!(
                "setjmp usage at line {} column {} in {:?}",
                line,
                column,
                file.unwrap().get_path().file_name().unwrap(),
            )),

            "longjmp" => warnings.push(format!(
                "longjmp usage at line {} column {} in {:?}",
                line,
                column,
                file.unwrap().get_path().file_name().unwrap()
            )),

            _ => continue,
        }
    }

    return warnings;
}
