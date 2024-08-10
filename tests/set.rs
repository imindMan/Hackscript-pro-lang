use hackscript::run;

#[test]
fn indexing_error() -> Result<(), String> {
    match run(String::from("{1, 2, 3}[2]")) {
        Ok(_) => Err(String::from("Shouldn't be passed")),
        Err(_) => Ok(()),
    }
}

#[test]
fn append_the_same_looking_element() {
    let res = match run(String::from("{1, 2, 3} +# \"3\"")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };

    assert_eq!(res, String::from("{1, 2, 3, \"3\", }"))
}

#[test]
fn append_duplicate_element() {
    let res = match run(String::from("{1, 2, 3} +# 2")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };

    assert_eq!(res, String::from("{1, 2, 3, }"))
}

#[test]
fn initialize_duplicate_elements_in_set() {
    let res = match run(String::from("{1, 2, 3, 2, 3}")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };

    assert_eq!(res, String::from("{1, 2, 3, }"))
}
