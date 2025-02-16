use hackscript::run::run;

#[test]
fn and_operator_check_for_number() -> Result<(), String> {
    match run(String::from("2 && 3")) {
        Ok(_ok) => Err("Shouldn't be passed".to_string()),
        Err(_err) => Ok(()),
    }
}

#[test]
fn or_operator_check_for_number() -> Result<(), String> {
    match run(String::from("2 || 3")) {
        Ok(_ok) => Err("Shouldn't be passed".to_string()),
        Err(_err) => Ok(()),
    }
}

#[test]
fn and_operator_check_for_string() -> Result<(), String> {
    match run(String::from("\"s\" && \"\"")) {
        Ok(_ok) => Err("Shouldn't be passed".to_string()),
        Err(_err) => Ok(()),
    }
}

#[test]
fn or_operator_check_for_string() -> Result<(), String> {
    match run(String::from("\"sss\" || \"ss\"")) {
        Ok(_ok) => Err("Shouldn't be passed".to_string()),
        Err(_err) => Ok(()),
    }
}
#[test]
fn and_operator_check_for_null() -> Result<(), String> {
    match run(String::from("null && null")) {
        Ok(_ok) => Err("Shouldn't be passed".to_string()),
        Err(_err) => Ok(()),
    }
}

#[test]
fn or_operator_check_for_null() -> Result<(), String> {
    match run(String::from("null || null")) {
        Ok(_ok) => Err("Shouldn't be passed".to_string()),
        Err(_err) => Ok(()),
    }
}
#[test]
fn equal_operator_check_for_number() {
    let run_res: String = match run(String::from("1 + 2 == 3")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };
    assert_eq!("true".to_string(), run_res)
}
#[test]
fn equal_operator_check_for_string() {
    let run_res: String = match run(String::from("\"ss\" == \"sss\"")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };
    assert_eq!("false".to_string(), run_res)
}
#[test]
fn equal_operator_check_for_null() {
    let run_res: String = match run(String::from("null == null")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };
    assert_eq!("true".to_string(), run_res)
}
#[test]
fn equal_operator_check_for_boolean() {
    let run_res: String = match run(String::from("true == true")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };
    assert_eq!("true".to_string(), run_res)
}
#[test]
fn not_equal_operator_check_for_number() {
    let run_res: String = match run(String::from("1 + 2 != 3")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };
    assert_eq!("false".to_string(), run_res)
}
#[test]
fn not_equal_operator_check_for_string() {
    let run_res: String = match run(String::from("\"ss\" != \"sss\"")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };
    assert_eq!("true".to_string(), run_res)
}
#[test]
fn not_equal_operator_check_for_boolean() {
    let run_res: String = match run(String::from("true != false")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };
    assert_eq!("true".to_string(), run_res)
}
#[test]
fn not_equal_operator_check_for_null() {
    let run_res: String = match run(String::from("null != null")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };
    assert_eq!("false".to_string(), run_res)
}
#[test]
fn greater_operator_check_for_number() {
    let run_res: String = match run(String::from("1 + 2 > 3")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };
    assert_eq!("false".to_string(), run_res)
}
#[test]
fn greater_operator_check_for_string() -> Result<(), String> {
    match run(String::from("\"ss\" > \"sss\"")) {
        Ok(_ok) => Err("Shouldn't be passed".to_string()),
        Err(_err) => Ok(()),
    }
}
#[test]
fn greater_operator_check_for_boolean() -> Result<(), String> {
    match run(String::from("true > false")) {
        Ok(_ok) => Err("Shouldn't be passed".to_string()),
        Err(_err) => Ok(()),
    }
}
#[test]
fn greater_operator_check_for_null() -> Result<(), String> {
    match run(String::from("null > null")) {
        Ok(_ok) => Err("Shouldn't be passed".to_string()),
        Err(_err) => Ok(()),
    }
}

#[test]
fn less_operator_check_for_number() {
    let run_res: String = match run(String::from("1 + 2 < 4")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };
    assert_eq!("true".to_string(), run_res)
}
#[test]
fn less_operator_check_for_string() -> Result<(), String> {
    match run(String::from("\"ss\" < \"sss\"")) {
        Ok(_ok) => Err("Shouldn't be passed".to_string()),
        Err(_err) => Ok(()),
    }
}
#[test]
fn less_operator_check_for_boolean() -> Result<(), String> {
    match run(String::from("true < false")) {
        Ok(_ok) => Err("Shouldn't be passed".to_string()),
        Err(_err) => Ok(()),
    }
}
#[test]
fn less_operator_check_for_null() -> Result<(), String> {
    match run(String::from("null < null")) {
        Ok(_ok) => Err("Shouldn't be passed".to_string()),
        Err(_err) => Ok(()),
    }
}

#[test]
fn less_or_equal_operator_check_for_number() {
    let run_res: String = match run(String::from("1 + 2 <= 3")) {
        Ok(ok) => format!("{}", ok),
        Err(err) => format!("{}", err),
    };
    assert_eq!("true".to_string(), run_res)
}
#[test]
fn less_or_equal_operator_check_for_string() -> Result<(), String> {
    match run(String::from("\"ss\" < \"sss\"")) {
        Ok(_ok) => Err("Shouldn't be passed".to_string()),
        Err(_err) => Ok(()),
    }
}
#[test]
fn less_or_equal_operator_check_for_boolean() -> Result<(), String> {
    match run(String::from("true < false")) {
        Ok(_ok) => Err("Shouldn't be passed".to_string()),
        Err(_err) => Ok(()),
    }
}
#[test]
fn less_or_equal_operator_check_for_null() -> Result<(), String> {
    match run(String::from("null < null")) {
        Ok(_ok) => Err("Shouldn't be passed".to_string()),
        Err(_err) => Ok(()),
    }
}
