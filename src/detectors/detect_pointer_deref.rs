use clang::*;

pub fn detect_pointer_deref(tu: &TranslationUnit) {
    let _ = tu.get_entity().visit_children(|child, parent| {
        if (parent.get_kind() == EntityKind::VarDecl || child.get_kind() == EntityKind::FieldDecl)
            && child.get_type().unwrap().get_display_name().contains("**")
        {
            println!(
                "Double deref detected at {:?}",
                child.get_location().unwrap().get_spelling_location()
            );
        }

        EntityVisitResult::Recurse
    });
}
