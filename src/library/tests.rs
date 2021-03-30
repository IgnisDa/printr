use super::*;
use std::error::Error;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_new_being_created_with_filename_supplied() -> Result<(), Box<dyn Error>> {
    let mut file = NamedTempFile::new()?;
    let content = "A test";
    write!(file, "{}", content)?;
    let printr = Printr::new(
        true,
        true,
        false,
        false,
        Some(file.path().to_str().unwrap().to_string()),
        None,
        Some("yellow".to_string()),
    );
    assert_eq!(
        printr,
        Printr {
            interpretations: true,
            newline: true,
            color: Some("yellow".to_string()),
            spaces: false,
            string: content.to_string()
        }
    );
    Ok(())
}

#[test]
fn test_new_being_created_with_input_string_supplied() -> Result<(), Box<dyn Error>> {
    let content = String::from("A test");
    let printr = Printr::new(true, true, false, false, None, Some(content.clone()), None);
    assert_eq!(
        printr,
        Printr {
            interpretations: true,
            newline: true,
            color: None,
            spaces: false,
            string: content.clone()
        }
    );
    Ok(())
}
