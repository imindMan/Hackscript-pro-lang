// INFO: This is the file for the AST
// NOTE: To understand what are the roles of those nodes in the AST, head over to src/parser/lib.rs
//
use lexer::Token;

#[derive(Debug, Clone)]
pub enum AST {
    Factor {
        identifier: String,
        token: Token,
    },

    UnaryFactor {
        sign: String,
        value: Box<AST>,
    },

    FormingCalc {
        node1: Box<AST>,
        operator: Option<Token>,
        node2: Box<AST>,
    },

    Nil,
}

impl AST {
    // This is the initialization method by default
    pub fn default() -> AST {
        AST::Nil
    }

    pub fn new_factor(token: Token) -> AST {
        let mut identifier: String = String::new();
        if token.value.contains('.') {
            identifier.push_str("float");
        } else {
            identifier.push_str("integer");
        }

        AST::Factor { identifier, token }
    }

    pub fn new_formingcalc(node1: Box<AST>, operator: Option<Token>, node2: Box<AST>) -> AST {
        AST::FormingCalc {
            node1,
            operator,
            node2,
        }
    }
    pub fn new_unaryfactor(sign: String, value: Box<AST>) -> AST {
        AST::UnaryFactor { sign, value }
    }
}
