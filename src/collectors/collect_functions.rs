use clang::*;

pub fn collect_functions<'a>(tu: &'a TranslationUnit<'a>) -> Vec<clang::Entity<'a>> {
    return tu
        .get_entity()
        .get_children()
        .into_iter()
        .filter(|e| e.get_kind() == EntityKind::FunctionDecl)
        .collect::<Vec<_>>();
}
