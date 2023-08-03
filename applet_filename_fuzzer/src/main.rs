#[macro_use]
extern crate afl;

fn main() {
    fuzz!(|data: &[u8]| {
        if let Ok(s) = std::str::from_utf8(data) {
            let _ = validate(s);
        }
    });
}

fn validate(filename: &str) -> Result<(), &'static str> {
    use regex::Regex;
    const MAX_LENGTH: usize = 512;

    // let filename = &*(*self);

    if filename.is_empty() {
        return Err("Invalid filename: It's empty");
    }
    if filename.len() > MAX_LENGTH {
        return Err("Invalid filename: More than 512 Bytes");
    }
    // TODO: Implement regex to match Filename as `file_name.lua`
    // TODO: '_' is optional.
    // TODO: '.' should exists only one.
    // TODO: '.lua$' is required

    // Generated with `Rustexp` on https://rustexp.lpil.uk/
    // TODO: Write tests
    let file_name_regex: Regex = Regex::new(r"^([a-z]{1}[a-z0-9_]{0,60})$").unwrap();

    let mut char_counter = 0;

    // * Check if is a valid ascii string
    for c in filename.chars() {
        // * ASCII only
        if !c.is_ascii() {
            return Err(
                "Invalid filename: Must have ONLY ascii alphanumeric characters OR '_' char",
            );
        }

        // * First char should be alphabetic lowercase
        if char_counter == 0 && !c.is_ascii_lowercase() {
            return Err("Invalid filename: Must start with lowercase character");
        }

        // * Valid inputs: "a-z", "0-9", '_' '.'
        if !(c.is_ascii_lowercase() || c.is_ascii_digit() || (c == '_') || (c == '.')) {
            return Err("Invalid filename: It has invalid characters");
        }
        char_counter += 1;
    }
    // * Must have one '.' only
    if !(filename.matches('.').count() == 1) {
        return Err("Invalid filename: Must have only one '.' char");
    }

    let file_name_vec: Vec<&str> = filename.split(".").collect();
    let name = file_name_vec[0];
    let extension = file_name_vec[1];

    // * Extension should be ".lua"
    if !(extension == "lua") {
        return Err("Invalid filename: The filename must ends with '.lua' extension");
    }

    // * Regex for a valid name for a file
    // The name of a file must have ONLY:
    // - ascii alphanumeric characters
    //
    //   OR
    //
    // - '_' char
    //
    if !file_name_regex.is_match(name) {
        return Err(
            "Invalid filename: The name of a file must have ONLY ascii alphanumeric characters OR '_' char");
    }
    Ok(())
}
