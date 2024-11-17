use clang::source::Location;
use clang::source::SourceLocation;

#[derive(Debug)]
pub struct ViolationLocation {
    pub line: u32,
    pub column: u32,
    pub filename: String,
}

pub fn get_violation_loc(
    violation_src: &Option<SourceLocation>,
) -> Result<ViolationLocation, &'static str> {
    let location = if let Some(src) = violation_src {
        src.get_spelling_location()
    } else {
        return Err("Failed to get violation source".into());
    };

    let Location {
        line, column, file, ..
    } = location;

    let filename = file
        .and_then(|f| f.get_path().file_name()?.to_str().map(|s| s.to_owned()))
        .ok_or("Failed to get filename")?;

    Ok(ViolationLocation {
        line,
        column,
        filename,
    })
}
