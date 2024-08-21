use clang::*;

use crate::collectors::collect_loops::collect_loops;
use source::Location;

// TODO make a helper function to get the line, column, and file name of a violation
pub fn detect_unbound_loops(tu: &TranslationUnit) {
    let loops = collect_loops(tu);
    for _loop in loops {
        match _loop.get_kind() {
            EntityKind::WhileStmt => {
                if !is_bound(_loop) {
                    let Location {
                        line, column, file, ..
                    } = _loop.get_location().unwrap().get_spelling_location();
                    println!(
                        "While loop at line {} column {} is unbounded in {:?}",
                        line,
                        column,
                        file.unwrap().get_path().file_name().unwrap()
                    );
                }
            }

            EntityKind::ForStmt => {
                if !is_bound(_loop) {
                    let Location {
                        line, column, file, ..
                    } = _loop.get_location().unwrap().get_spelling_location();
                    println!(
                        "For loop at line {} column {} is unbounded in {:?}",
                        line,
                        column,
                        file.unwrap().get_path().file_name().unwrap()
                    );
                }
            }
            _ => return,
        }
    }
}

fn is_bound(_loop: Entity) -> bool {
    return _loop
        .get_children()
        .into_iter()
        .any(|c| c.get_kind() == EntityKind::BinaryOperator);
}
