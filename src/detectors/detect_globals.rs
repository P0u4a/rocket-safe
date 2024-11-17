use clang::*;
use source::SourceLocation;

use super::helpers::{get_violation_loc, ViolationLocation};

pub fn detect_globals(tu: &TranslationUnit) -> Vec<String> {
    let mut globals: Vec<(Option<SourceLocation>, String)> = vec![];
    let mut warnings: Vec<String> = vec![];

    tu.get_entity().get_children().into_iter().for_each(|e| {
        if e.get_kind() == EntityKind::VarDecl && !e.is_in_system_header() {
            globals.push((e.get_location(), e.get_name().unwrap()))
        }
    });

    for (loc, name) in globals {
        let ViolationLocation {
            line,
            column,
            filename,
        } = match get_violation_loc(&loc) {
            Ok(location) => location,
            Err(err) => {
                warnings.push(format!(
                    "Global variable declared at unknown location. {}",
                    err
                ));
                continue;
            }
        };

        warnings.push(format!(
            "Global variable {} declared at line {} column {} in {:?}",
            name, line, column, filename
        ));
    }

    return warnings;
}
