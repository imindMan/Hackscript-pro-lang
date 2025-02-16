use hackscript::run::run;

#[test]
fn indexing_out_of_range() -> Result<(), String> {
    match run(String::from("(0, 1)[2]")) {
        Ok(_) => Err(String::from("Shouldn't be passed")),
        Err(_) => Ok(()),
    }
}
#[test]
fn append_test() -> Result<(), String> {
    match run(String::from("(0, 1, 2) +# 22")) {
        Ok(_) => Err(String::from("Shouldn't be passed")),
        Err(_) => Ok(()),
    }
}

#[test]
fn error_on_normal_equation() -> Result<(), String> {
    match run(String::from("(0, 1, 2) + 23")) {
        Ok(_) => Err(String::from("Shouldn't be passed")),
        Err(_) => Ok(()),
    }
}

#[test]
fn tracing_element_through_index() {
    let res = match run(String::from("(0, 1, 2, 3)[2]")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };

    assert_eq!(res, String::from("2"));
}
