use clang::*;

use super::helpers::get_violation_loc;

pub fn detect_pointer_deref(tu: &TranslationUnit) -> Vec<String> {
    let mut warnings: Vec<String> = vec![];

    let _ = tu.get_entity().visit_children(|child, parent| {
        if (parent.get_kind() == EntityKind::VarDecl || child.get_kind() == EntityKind::FieldDecl)
            && child.get_type().unwrap().get_display_name().contains("**")
        {
            match get_violation_loc(&child.get_location()) {
                Ok(location) => {
                    warnings.push(format!(
                        "Double pointer dereference detected at line {} column {} in {:?}",
                        location.line, location.column, location.filename
                    ));
                }
                Err(err) => {
                    warnings.push(format!(
                        "Double pointer dereference at unknown location. {}",
                        err
                    ));
                }
            };
        }

        EntityVisitResult::Recurse
    });

    return warnings;
}
