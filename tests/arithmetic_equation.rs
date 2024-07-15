use hackscript::run;

#[test]
fn simple_adding_positive_equation() {
    let run_res: String = match run(String::from("3 + 2")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };

    assert_eq!("5\n".to_string(), run_res);
}

#[test]
fn simple_subtract_positive_equation() {
    let run_res: String = match run(String::from("3 - 2")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };

    assert_eq!("1\n".to_string(), run_res);
}

#[test]
fn simple_multiply_positive_equation() {
    let run_res: String = match run(String::from("3 * 2")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };

    assert_eq!("6\n".to_string(), run_res);
}

#[test]
fn simple_divide_positive_equation() {
    let run_res: String = match run(String::from("4 / 2")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };

    assert_eq!("2\n".to_string(), run_res);
}
