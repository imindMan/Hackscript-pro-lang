// INFO: some types of errors are gonna defined here
// To make sure that everything is neat, I'm gonna make a universal function to
// handle every single types of errors

// universal function to handle the error types
pub fn error_type_handling(_type: (String, String)) -> String {
    // match checking the _type
    let error_type: String;
    match _type.0.as_str() {
        "Undefined character" => error_type = undefined_character(_type.0.clone(), _type.1.clone()),
        "Number error" => error_type = number_error(_type.0.clone()),
        "Expect" => error_type = expect(_type.0.clone(), _type.1.clone()),
        _ => panic!("Unspecified error type"),
    }
    error_type
}

// -----------------------------------------------------------
// EVERY SINGLE ERROR TYPE-DEFINING FUNCTIONS ARE DEFINED HERE
// -----------------------------------------------------------

// Undefined character type
fn undefined_character(_type: String, character_not_defined: String) -> String {
    let error_string: String =
        "Error type: ".to_string() + &_type + "\"" + &character_not_defined + "\"";
    error_string
}

fn number_error(_type: String) -> String {
    let error_string: String =
        "Error type: ".to_string() + &_type + ". This number cannot be defined";
    error_string
}

fn expect(_type: String, string: String) -> String {
    let error_string: String =
        "Error type: ".to_string() + &_type + "\n" + "What to expect: " + &string;
    error_string
}
