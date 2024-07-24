// INFO: MAINLY FOR THE LEXER
// These tests will check if the lexer is working or not
use hackscript::run;

#[test]
fn backspace_too_much() {
    let run_res: String = match run(String::from("                      ")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };

    assert_eq!("".to_string(), run_res);
}

#[test]

fn newline_too_much() {
    let run_res: String = match run(String::from("\n\n\n\n\n\n\n\n\n\n\n\n\n")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };

    assert_eq!("".to_string(), run_res);
}

#[test]

// For now, we only accept arithmetic equation
fn failing_character() -> Result<(), String> {
    match run(String::from("a")) {
        Ok(_ok) => Err("Shouldn't be passed".to_string()),
        Err(_err) => Ok(()),
    }
}

#[test]
fn failing_character_with_backscape() -> Result<(), String> {
    match run(String::from("   a-    ")) {
        Ok(_ok) => Err("Shouldn't be passed".to_string()),
        Err(_err) => Ok(()),
    }
}

#[test]
fn failing_character_with_newline() -> Result<(), String> {
    match run(String::from("\n\n\n\n\nabc\n\n\n")) {
        Ok(_ok) => Err("Shouldn't be passed".to_string()),
        Err(_err) => Ok(()),
    }
}

#[test]
fn failing_character_with_backscape_and_newline() -> Result<(), String> {
    match run(String::from("   \n \n \n a\n b - dd c  \n \n ")) {
        Ok(_ok) => Err("Shouldn't be passed".to_string()),
        Err(_err) => Ok(()),
    }
}
