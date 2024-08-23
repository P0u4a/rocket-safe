use clang::*;
use source::Location;

pub fn detect_globals(tu: &TranslationUnit) -> Vec<String> {
    let mut globals: Vec<(Location, String)> = vec![];
    let mut warnings: Vec<String> = vec![];

    tu.get_entity().get_children().into_iter().for_each(|e| {
        if e.get_kind() == EntityKind::VarDecl && !e.is_in_system_header() {
            globals.push((
                e.get_location().unwrap().get_spelling_location(),
                e.get_name().unwrap(),
            ))
        }
    });

    for (loc, name) in globals {
        let Location {
            line, column, file, ..
        } = loc;

        warnings.push(format!(
            "Global variable {} declared at line {} column {} in {:?}",
            name,
            line,
            column,
            file.unwrap().get_path().file_name().unwrap()
        ));
    }

    return warnings;
}
