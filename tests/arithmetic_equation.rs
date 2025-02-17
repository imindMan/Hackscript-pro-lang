// INFO: These are tests for arithmetic operation
// (For now, it's just Numbers, but in the future, there will be more)
// arithmetic here is just **exclusively** for plus, minus, multiply, and divide
// operation
use hackscript::run::run;

#[test]
fn lexer_err_checking_unknown_character() -> Result<(), String> {
    match run(String::from("1 + 2a\n")) {
        Ok(_ok) => Err("Shouldn't be passed".to_string()),
        Err(_err) => Ok(()),
    }
}

#[test]
fn parser_err_checking_parentheses() -> Result<(), String> {
    match run(String::from("((1 + 2)")) {
        Ok(_ok) => Err("Shouldn't be passed".to_string()),
        Err(_err) => Ok(()),
    }
}
#[test]
fn parser_err_checking_number() -> Result<(), String> {
    match run(String::from("1 + ")) {
        Ok(_ok) => Err("Shouldn't be passed".to_string()),
        Err(_err) => Ok(()),
    }
}
#[test]
fn parser_err_checking_number_factor() -> Result<(), String> {
    match run(String::from("1 + 2 *")) {
        Ok(_ok) => Err("Shouldn't be passed".to_string()),
        Err(_err) => Ok(()),
    }
}
#[test]
fn parser_err_checking_operator() -> Result<(), String> {
    match run(String::from("1 (2")) {
        Ok(_ok) => Err("Shouldn't be passed".to_string()),
        Err(_err) => Ok(()),
    }
}
#[test]
fn float_checking_empty() -> Result<(), String> {
    match run(String::from("1.")) {
        Ok(_ok) => Err("Shouldn't be passed".to_string()),
        Err(_err) => Ok(()),
    }
}
#[test]
fn float_checking_duplicate() -> Result<(), String> {
    match run(String::from("1..2")) {
        Ok(_ok) => Err("Shouldn't be passed".to_string()),
        Err(_err) => Ok(()),
    }
}
// special implementation for float
#[test]
fn empty_before_dot_float() {
    let run_res: String = match run(String::from(".2")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };
    assert_eq!("0.2".to_string(), run_res);
}

#[test]
fn simple_adding_positive_equation() {
    let run_res: String = match run(String::from("3 + 2")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };

    assert_eq!("5".to_string(), run_res);
}

#[test]
fn simple_subtract_positive_equation() {
    let run_res: String = match run(String::from("3 - 2")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };

    assert_eq!("1".to_string(), run_res);
}

#[test]
fn simple_multiply_positive_equation() {
    let run_res: String = match run(String::from("3 * 2")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };

    assert_eq!("6".to_string(), run_res);
}

#[test]
fn simple_divide_positive_equation() {
    let run_res: String = match run(String::from("4 / 2")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };

    assert_eq!("2".to_string(), run_res);
}

#[test]
fn equation_with_two_factors_plus() {
    let run_res: String = match run(String::from("1 * 2 + 3 / 2")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };

    assert_eq!("3.5".to_string(), run_res);
}

#[test]
fn equation_with_two_factors_plus_parentheses() {
    let run_res: String = match run(String::from("1 * 2 + (3 / 2 + 2.5)")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };

    assert_eq!("6".to_string(), run_res);
}
#[test]
fn equation_with_two_factors_minus() {
    let run_res: String = match run(String::from("1 * 2 - 3 / 2")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };

    assert_eq!("0.5".to_string(), run_res);
}

#[test]
fn equation_with_two_factors_minus_parentheses() {
    let run_res: String = match run(String::from("4 * 2 - (2.5 + 4)")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };

    assert_eq!("1.5".to_string(), run_res);
}
#[test]
fn equation_with_two_factors_multiply() {
    let run_res: String = match run(String::from("(2 + 3)*(4 + 6)")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };

    assert_eq!("50".to_string(), run_res);
}

#[test]
fn equation_with_two_factors_divide() {
    let run_res: String = match run(String::from("(2 * 6)/(3 * 4)")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };

    assert_eq!("1".to_string(), run_res);
}

#[test]
fn divide_by_zero_error_check() -> Result<(), String> {
    match run("1/0".to_string()) {
        Ok(_ok) => Err("Shouldn't be passed".to_string()),
        Err(_err) => Ok(()),
    }
}

#[test]

fn more_complicated_standalone_plus_equation() {
    let run_res: String = match run(String::from("2 + 3 + 4")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };
    assert_eq!("9".to_string(), run_res);
}

#[test]

fn more_complicated_compound_plus_equation() {
    let run_res: String = match run(String::from("2*3 + 3 * 4 + (2 + 3 * 4)")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };
    assert_eq!("32".to_string(), run_res);
}

#[test]

fn more_complicated_standalone_minus_equation() {
    let run_res: String = match run(String::from("3 -2 -1")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };
    assert_eq!("0".to_string(), run_res);
}

#[test]

fn more_complicated_compound_minus_equation() {
    let run_res: String = match run(String::from("2*3 - 3 * 4 + (2 + 3 * 4)")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };
    assert_eq!("8".to_string(), run_res);
}

#[test]

fn more_complicated_standalone_multiply_equation() {
    let run_res: String = match run(String::from("2 * 3 * 4")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };
    assert_eq!("24".to_string(), run_res);
}

#[test]

fn more_complicated_compound_multiply_equation() {
    let run_res: String = match run(String::from("(2 + 3)*(4 - 6)")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };
    assert_eq!("-10".to_string(), run_res);
}

#[test]

fn more_complicated_standalone_divide_equation() {
    let run_res: String = match run(String::from("625/25/25")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };
    assert_eq!("1".to_string(), run_res);
}

#[test]

fn more_complicated_compound_divide_equation() {
    let run_res: String = match run(String::from("(26 * 4)/(4 * 2)/(4 * 2)")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };
    assert_eq!("1.625".to_string(), run_res);
}
