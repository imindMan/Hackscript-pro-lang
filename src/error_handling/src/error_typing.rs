// INFO: some types of errors are gonna defined here
// To make sure that everything is neat, I'm gonna make a universal function to
// handle every single types of errors

// INFO: universal function to handle the error types
pub fn error_type_handling(_type: (String, String)) -> String {
    // match checking the _type
    let error_type = match _type.0.as_str() {
        "Undefined character" => undefined_character(_type.0.clone(), _type.1.clone()),
        "Number error" => number_error(_type.0.clone(), _type.1.clone()),
        "Expect" => expect(_type.0.clone(), _type.1.clone()),
        "DivisionByZero" => divisonbyzero(_type.0.clone(), _type.1.clone()),
        _ => panic!("Unspecified error type"),
    };
    error_type
}

// -----------------------------------------------------------
// EVERY SINGLE ERROR TYPE-DEFINING FUNCTIONS ARE DEFINED HERE
// -----------------------------------------------------------

fn undefined_character(_type: String, character_not_defined: String) -> String {
    let error_string: String =
        "Error type: ".to_string() + &_type + " \"" + &character_not_defined + "\"";
    error_string
}

fn number_error(_type: String, number_not_defined: String) -> String {
    let error_string: String = "Error type: ".to_string()
        + &_type
        + ". This number cannot be defined: "
        + &number_not_defined;
    error_string
}

fn expect(_type: String, string: String) -> String {
    let error_string: String = "Error: ".to_string() + &_type + " " + &string;
    error_string
}

fn divisonbyzero(_type: String, string: String) -> String {
    let error_string: String = "Error: ".to_string() + &_type + ". " + &string;
    error_string
}
