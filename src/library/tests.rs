use super::*;
use std::error::Error;
use std::io::Write;
use tempfile::NamedTempFile;

// we keep using the word "zealous" here since we know it's
// [afinn](https://github.com/fnielsen/afinn) score is 2.0 and can be used consistently for
// tests

#[test]
fn test_new_being_created_with_filename_supplied() -> Result<(), Box<dyn Error>> {
    let mut file = NamedTempFile::new()?;
    let content = "zealous";
    write!(file, "{}", content)?;
    let printr = Printr::new(
        true,
        true,
        false,
        false,
        Some(file.path().to_str().unwrap().to_string()),
        None,
        None,
        true,
    );
    assert_eq!(
        printr,
        Printr {
            interpretations: true,
            newline: true,
            color: Some("green".to_string()),
            spaces: false,
            string: content.to_string(),
            sentiment: Sentiment(2.0, 0.0),
            error: true
        }
    );
    Ok(())
}

#[test]
fn test_new_being_created_with_input_string_supplied() -> Result<(), Box<dyn Error>> {
    let content = String::from("zealous");
    let printr = Printr::new(
        true,
        true,
        false,
        false,
        None,
        None,
        Some(content.clone()),
        true,
    );
    assert_eq!(
        printr,
        Printr {
            interpretations: true,
            newline: true,
            color: Some("green".to_string()),
            spaces: false,
            string: content.clone(),
            sentiment: Sentiment(2.0, 0.0),
            error: true
        }
    );
    Ok(())
}
