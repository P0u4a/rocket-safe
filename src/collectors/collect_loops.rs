use clang::*;

pub fn collect_loops<'a>(tu: &'a TranslationUnit<'a>) -> Vec<clang::Entity<'a>> {
    let mut loops: Vec<Entity> = vec![];

    let _ = tu.get_entity().visit_children(|child, _parent| {
        if child.get_kind() == EntityKind::ForStmt || child.get_kind() == EntityKind::WhileStmt {
            loops.push(child);
        }
        EntityVisitResult::Recurse
    });

    return loops;
}
