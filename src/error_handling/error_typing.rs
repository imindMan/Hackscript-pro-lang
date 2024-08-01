// INFO: some types of errors are gonna defined here
// To make sure that everything is neat, I'm gonna make a universal function to
// handle every single types of errors

// INFO: universal function to handle the error types
use inline_colorization::*;

pub fn error_type_handling(info: (String, String)) -> String {
    // match checking the _type
    match info.0.as_str() {
        "Undefined character" => undefined_character(info.0.clone(), info.1.clone()),
        "Number error" => number_error(info.0.clone(), info.1.clone()),
        "Expect" => expect(info.0.clone(), info.1.clone()),
        "DivisionByZero" => divisionbyzero(info.0.clone(), info.1.clone()),
        "TypeError" => typeerror(info.0.clone(), info.1.clone()),
        "UnknownTrailingCharacter" => unknowntrailingcharacter(info.0.clone(), info.1.clone()),
        "ValueError" => valueerror(info.0.clone(), info.1.clone()),
        "UnidentifiedIdentifier" => unidentifiedidentifier(info.0.clone(), info.1.clone()),
        "OperatorError" => operatorerror(info.0.clone(), info.1.clone()),
        _ => panic!("Unspecified error type"),
    }
}

// -----------------------------------------------------------
// EVERY SINGLE ERROR TYPE-DEFINING FUNCTIONS ARE DEFINED HERE
// -----------------------------------------------------------

fn undefined_character(_type: String, character_not_defined: String) -> String {
    let error_string: String = format!(
        "{}{}Error type: {}{}",
        style_bold, color_red, style_reset, color_reset
    ) + style_underline
        + &_type
        + style_reset
        + style_bold
        + color_yellow
        + " \""
        + &character_not_defined
        + "\""
        + style_reset
        + color_reset;
    error_string
}

fn number_error(_type: String, number_not_defined: String) -> String {
    let error_string: String = format!(
        "{}{}Error type: {}{}",
        style_bold, color_red, style_reset, color_reset
    ) + style_underline
        + &_type
        + style_reset
        + ". This number cannot be defined: "
        + style_bold
        + color_yellow
        + &number_not_defined
        + style_reset
        + color_reset;
    error_string
}

fn expect(_type: String, string: String) -> String {
    let error_string: String = format!(
        "{}{}Error: {}{}",
        style_bold, color_red, style_reset, color_reset
    ) + style_underline
        + &_type
        + style_reset
        + " "
        + &string;
    error_string
}

fn divisionbyzero(_type: String, string: String) -> String {
    let error_string: String = format!(
        "{}{}Error: {}{}",
        style_bold, color_red, style_reset, color_reset
    ) + style_underline
        + &_type
        + style_reset
        + ". "
        + &string;
    error_string
}
fn operatorerror(_type: String, string: String) -> String {
    let error_string: String = format!(
        "{}{}Error: {}{}",
        style_bold, color_red, style_reset, color_reset
    ) + style_underline
        + &_type
        + style_reset
        + ". "
        + &string;
    error_string
}
fn typeerror(_type: String, string: String) -> String {
    let error_string: String = format!(
        "{}{}Error type: {}{}",
        style_bold, color_red, style_reset, color_reset
    ) + style_underline
        + &_type
        + style_reset
        + ". "
        + &string;
    error_string
}

fn unknowntrailingcharacter(_type: String, string: String) -> String {
    let error_string: String = format!(
        "{}{}Error type: {}{}",
        style_bold, color_red, style_reset, color_reset
    ) + style_underline
        + &_type
        + style_reset
        + ". "
        + &string;
    error_string
}

fn valueerror(_type: String, string: String) -> String {
    let error_string: String = format!(
        "{}{}Error type: {}{}",
        style_bold, color_red, style_reset, color_reset
    ) + style_underline
        + &_type
        + style_reset
        + ". "
        + &string;
    error_string
}

fn unidentifiedidentifier(_type: String, identifier_not_defined: String) -> String {
    let error_string: String = format!(
        "{}{}Error type: {}{}",
        style_bold, color_red, style_reset, color_reset
    ) + style_underline
        + &_type
        + style_reset
        + ". This identifier cannot be defined: "
        + style_bold
        + color_yellow
        + &identifier_not_defined
        + style_reset
        + color_reset;
    error_string
}
