use clang::*;
use source::Location;

pub fn detect_pointer_deref(tu: &TranslationUnit) -> Vec<String> {
    let mut warnings: Vec<String> = vec![];

    let _ = tu.get_entity().visit_children(|child, parent| {
        if (parent.get_kind() == EntityKind::VarDecl || child.get_kind() == EntityKind::FieldDecl)
            && child.get_type().unwrap().get_display_name().contains("**")
        {
            let Location {
                line, column, file, ..
            } = child.get_location().unwrap().get_spelling_location();
            warnings.push(format!(
                "Double pointer dereference detected at line {} column {} in {:?}",
                line,
                column,
                file.unwrap().get_path().file_name().unwrap()
            ));
        }

        EntityVisitResult::Recurse
    });

    return warnings;
}
