use hackscript::run;

#[test]

fn out_of_indexing_error() -> Result<(), String> {
    match run(String::from("[1, 2, 3][3]")) {
        Ok(_) => Err(String::from("Shouldn't be passed")),
        Err(_) => Ok(()),
    }
}

#[test]

fn invalid_operation() -> Result<(), String> {
    match run(String::from("[1, 2, 3, 4] + 2")) {
        Ok(_) => Err(String::from("Shouldn't be passed")),
        Err(_) => Ok(()),
    }
}

#[test]

fn append_some_stuff() {
    let res = match run(String::from("[1, 2, 3, 4] +# 2")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };

    assert_eq!(res, String::from("[1, 2, 3, 4, 2, ]"));
}

#[test]
fn index_some_stuff() {
    let res = match run(String::from("[1, 2, 3, 2][2]")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };

    assert_eq!(res, String::from("3"));
}
