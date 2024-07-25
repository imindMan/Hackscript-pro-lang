// INFO: All the general Hackscript types are defined here
//

// END-OF-FILE token
pub const EOF: &str = "EOF";
// Available characters
pub const AVAILABLE_CHARACTERS: &str = "abcdefghjklmnopqrstuvwxyzABCDEFGHJKLMNOPQRSTUVWXYZ";

// Operators

pub const PLUS: &str = "PLUS";
pub const MINUS: &str = "MINUS";
pub const MULTIPLY: &str = "MULTIPLY";
pub const DIVIDE: &str = "DIVIDE";

// Parentheses
pub const PARENTHESE_OPEN: &str = "PAREN_OPEN";
pub const PARENTHESE_CLOSE: &str = "PAREN_CLOSE";

// Number type
// In Hackscript, there's only one type called Number to handle integer and float
// This is because I just want to make everything short and quick, so that's why
// numbering shouldn't be so complicated

pub const NUMBER: &str = "NUMBER";
pub const NUMBERLIST: &str = "0123456789.";

// String type
// Now the problem is, RUST IS VERY STRICT AT "STRING"
// But, Hackscript doesn't really care about string that much
// So we will implement string to be much more loose instead of just following rust blindly

pub const STRING: &str = "STRING";

// Comparison operator
pub const EQUAL: &str = "EQUAL";
pub const GREATER: &str = "GREATER";
pub const LESS: &str = "LESS";
pub const GREATER_OR_EQUAL: &str = "GREATER_OR_EQUAL";
pub const LESS_OR_EQUAL: &str = "LESS_OR_EQUAL";
pub const NOT_EQUAL: &str = "NOT_EQUAL";

// Booleans
pub const TRUE: &str = "TRUE";
pub const FALSE: &str = "FALSE";
pub const NULL: &str = "NULL";

// all the available keyword for the programming language right now
pub const AVAILABLE_KEYWORDS: &[(&str, &str)] = &[("true", TRUE), ("false", FALSE), ("null", NULL)];
