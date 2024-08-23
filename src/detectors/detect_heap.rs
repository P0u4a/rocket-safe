use clang::*;
use source::Location;

pub fn detect_heap(tu: &TranslationUnit) -> Vec<String> {
    let mut heap_uses: Vec<(Location, String)> = vec![];

    let _ = tu.get_entity().visit_children(|child, _parent| {
        if child.get_kind() == EntityKind::CallExpr
            && (child.get_name().unwrap() == "free" || child.get_name().unwrap() == "malloc")
        {
            heap_uses.push((
                child.get_location().unwrap().get_spelling_location(),
                child.get_name().unwrap(),
            ));
            return EntityVisitResult::Continue;
        }
        EntityVisitResult::Recurse
    });

    let mut warnings: Vec<String> = vec![];

    for (use_loc, name) in heap_uses {
        let Location {
            line, column, file, ..
        } = use_loc;

        match name.as_str() {
            "malloc" => warnings.push(format!(
                "Dynamic memory allocation at line {} column {} in {:?}",
                line,
                column,
                file.unwrap().get_path().file_name().unwrap()
            )),

            "free" => warnings.push(format!(
                "Dynamic memory release at line {} column {} in {:?}",
                line,
                column,
                file.unwrap().get_path().file_name().unwrap()
            )),

            _ => continue,
        }
    }

    return warnings;
}
