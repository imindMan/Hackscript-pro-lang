// INFO: some types of errors are gonna defined here
// To make sure that everything is neat, I'm gonna make a universal function to
// handle every single types of errors

// INFO: universal function to handle the error types
pub fn error_type_handling(info: (String, String)) -> String {
    // match checking the _type
    match info.0.as_str() {
        "Undefined character" => undefined_character(info.0.clone(), info.1.clone()),
        "Number error" => number_error(info.0.clone(), info.1.clone()),
        "Expect" => expect(info.0.clone(), info.1.clone()),
        "DivisionByZero" => divisonbyzero(info.0.clone(), info.1.clone()),
        "TypeError" => typeerror(info.0.clone(), info.1.clone()),
        "OperationError" => operationerror(info.0.clone(), info.1.clone()),
        _ => panic!("Unspecified error type"),
    }
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

fn typeerror(_type: String, string: String) -> String {
    let error_string: String = "Error type: ".to_string() + &_type + " " + &string;
    error_string
}

fn operationerror(_type: String, string: String) -> String {
    let error_string: String = "Error type: ".to_string() + &_type + ". " + &string;
    error_string
}
