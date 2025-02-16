// error checking for the string first
use hackscript::run::run;

#[test]
fn endless_string_error() -> Result<(), String> {
    match run(String::from("\"add")) {
        Ok(_ok) => Err("Shouldn't be passed".to_string()),
        Err(_err) => Ok(()),
    }
}

#[test]
fn trailing_character_error() -> Result<(), String> {
    match run(String::from("\"\\a\"")) {
        Ok(_ok) => Err("Shouldn't be passed".to_string()),
        Err(_err) => Ok(()),
    }
}

#[test]
fn wrong_operation_subtract_error() -> Result<(), String> {
    match run(String::from("\"string\" - \"str\"")) {
        Ok(_ok) => Err("Shouldn't be passed".to_string()),
        Err(_err) => Ok(()),
    }
}

#[test]
fn wrong_operation_multiply_error() -> Result<(), String> {
    match run(String::from("\"string\" * \"str\"")) {
        Ok(_ok) => Err("Shouldn't be passed".to_string()),
        Err(_err) => Ok(()),
    }
}

#[test]
fn wrong_operation_divide_error() -> Result<(), String> {
    match run(String::from("\"string\" / \"str\"")) {
        Ok(_ok) => Err("Shouldn't be passed".to_string()),
        Err(_err) => Ok(()),
    }
}

// check the operation of string is working

#[test]
fn plus_operation() {
    let run_res: String = match run(String::from("\"string\" + \"string\"")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };

    assert_eq!("stringstring".to_string(), run_res);
}
#[test]
fn multiply_operation() {
    let run_res: String = match run(String::from("\"string\" * 3")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };

    assert_eq!("stringstringstring".to_string(), run_res);
}

#[test]
fn plus_complicated_operation() {
    let run_res: String = match run(String::from("\"string\" + \"hi\" + \"there\"")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };

    assert_eq!("stringhithere".to_string(), run_res);
}

#[test]
fn multiply_complicated_operation() {
    let run_res: String = match run(String::from("\"string\" * 3 * 2")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };

    assert_eq!("stringstringstringstringstringstring".to_string(), run_res);
}
#[test]
fn plus_multiply_complicated_operation() {
    let run_res: String = match run(String::from("\"string\" * 3 * 2 + \"hi\"")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };

    assert_eq!(
        "stringstringstringstringstringstringhi".to_string(),
        run_res
    );
}

#[test]
fn plus_parentheses_operation() {
    let run_res: String = match run(String::from("\"string\" + \"hi\" + (\"there\" + \"hey\")")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };

    assert_eq!("stringhitherehey".to_string(), run_res);
}

#[test]

fn plus_parentheses_operation_combined_multiply() {
    let run_res: String = match run(String::from(
        "\"string\" + \"hi\" + (\"there\" * 3+ \"hey\")",
    )) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };

    assert_eq!("stringhitheretheretherehey".to_string(), run_res);
}

#[test]

fn multiply_parentheses_operation() {
    let run_res: String = match run(String::from(" \"string\" * (3 - 2*1)")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };

    assert_eq!("string".to_string(), run_res);
}

#[test]
fn real_case_possible() {
    let run_res: String = match run(String::from("\"\\t\\t\\tSTRING IMPLEMENTATION\" + \"\\n\\n\" + \"After hard work, it's finally here\"+ \"\\n\\n\" + \"Now, let's \\\"get into it\\\"\"") ) {
        Ok(ok) => format!("{}", ok), 
        Err(err) => format!("{}", err),
    };

    assert_eq!("\t\t\tSTRING IMPLEMENTATION\n\nAfter hard work, it's finally here\n\nNow, let's \"get into it\"", run_res);
}
