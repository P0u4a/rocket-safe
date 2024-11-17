use clang::*;

use super::helpers::{get_violation_loc, ViolationLocation};
use crate::collectors::collect_loops::collect_loops;

pub fn detect_unbound_loops(tu: &TranslationUnit) -> Vec<String> {
    let loops = collect_loops(tu);
    let mut warnings: Vec<String> = vec![];

    for _loop in loops {
        match _loop.get_kind() {
            EntityKind::WhileStmt => {
                if !is_bound(_loop) {
                    let ViolationLocation {
                        line,
                        column,
                        filename,
                    } = match get_violation_loc(&_loop.get_location()) {
                        Ok(location) => location,
                        Err(err) => {
                            warnings
                                .push(format!("Unbounded while loop at unknown location. {}", err));
                            continue;
                        }
                    };
                    warnings.push(format!(
                        "While loop at line {} column {} is unbounded in {:?}",
                        line, column, filename
                    ));
                }
            }

            EntityKind::ForStmt => {
                if !is_bound(_loop) {
                    let ViolationLocation {
                        line,
                        column,
                        filename,
                    } = match get_violation_loc(&_loop.get_location()) {
                        Ok(location) => location,
                        Err(err) => {
                            warnings
                                .push(format!("Unbounded for loop at unknown location. {}", err));
                            continue;
                        }
                    };
                    warnings.push(format!(
                        "For loop at line {} column {} is unbounded in {:?}",
                        line, column, filename
                    ));
                }
            }
            _ => continue,
        }
    }

    return warnings;
}

fn is_bound(_loop: Entity) -> bool {
    return _loop
        .get_children()
        .into_iter()
        .any(|c| c.get_kind() == EntityKind::BinaryOperator);
}
