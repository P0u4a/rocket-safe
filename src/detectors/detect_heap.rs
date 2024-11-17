use clang::*;
use source::SourceLocation;

use super::helpers::{get_violation_loc, ViolationLocation};

pub fn detect_heap(tu: &TranslationUnit) -> Vec<String> {
    let mut heap_uses: Vec<(Option<SourceLocation>, String)> = vec![];

    let _ = tu.get_entity().visit_children(|child, _parent| {
        if child.get_kind() == EntityKind::CallExpr
            && (child.get_name().unwrap() == "free" || child.get_name().unwrap() == "malloc")
        {
            heap_uses.push((child.get_location(), child.get_name().unwrap()));
            return EntityVisitResult::Continue;
        }
        EntityVisitResult::Recurse
    });

    let mut warnings: Vec<String> = vec![];

    for (use_loc, name) in heap_uses {
        let ViolationLocation {
            line,
            column,
            filename,
        } = match get_violation_loc(&use_loc) {
            Ok(location) => location,
            Err(err) => {
                warnings.push(format!("Dynamic memory use at unknown location. {}", err));
                continue;
            }
        };

        match name.as_str() {
            "malloc" => warnings.push(format!(
                "Dynamic memory allocation at line {} column {} in {:?}",
                line, column, filename
            )),

            "free" => warnings.push(format!(
                "Dynamic memory release at line {} column {} in {:?}",
                line, column, filename
            )),

            _ => continue,
        }
    }

    return warnings;
}
