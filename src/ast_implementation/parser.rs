use crate::ast_implementation::Token;
use crate::ast_implementation::AST;
use crate::error_handling::Error;
use crate::hacktypes::*;

pub struct Parser {
    tokens: Vec<Token>,
    curr_tok: Token,
    curr_index: usize,
}

// INFO: General grammar rules with improvisation, this is the roles of all the AST nodes in
// Hackscript
// Factor (the smallest unit of Hackscript): Number
//         Unary: (PLUS||MINUS)((PLUS|MINUS) Number)*
//         LEFT_PAREN Expr (COMMA Expr)* RIGHT_PAREN
//         String
//         booleans (true, false)
// Term: Factor ((MUL||DIV) Factor)*
// Arithmetic_expr: Term ((PLUS||MINUS) Term)*
// Comp_expr: Arithmetic_expr ((GREATER|LESS|GREATER_OR_EQUAL|LESS_OR_EQUAL|EQUAL|NOT_EQUAL) Arithmetic_expr)*
// *Expr: Comp_expr ((AND|OR) Comp_expr)*

impl Parser {
    // INFO: This is the initialization method of the Parser
    pub fn new(tokens: Vec<Token>) -> Parser {
        let curr_index: usize = 0;
        let curr_tok = tokens[curr_index].clone();
        Parser {
            tokens,
            curr_index,
            curr_tok,
        }
    }

    // since the parser only has one function, that is to parse tokens into AST, this is the only
    // method beside the init method that is publicized

    // INFO: After initializing the parser, this function will parse the tokens and return the AST
    // for the interpreter
    pub fn parse(&mut self) -> Result<AST, Error> {
        if self.curr_tok._type == EOF {
            return Ok(AST::Nil);
        }

        // for now the FormingCalc AST (see the src/ast for more information) is the top node
        let expr = self.expr()?;
        if self.curr_tok._type != EOF {
            return Err(Error::new(
                "Expect".to_string(),
                "an operator like '+', '-', '*' or '/'".to_string(),
                self.curr_tok.pos_start.clone(),
                self.curr_tok.pos_end.clone(),
            ));
        }
        Ok(expr)
    }

    // ------------------------------------------------------------------
    // INFO: Below (until the next block of comment like this)
    // are the necessary methods that are generally used in the parser
    // -----------------------------------------------------------------

    fn advance(&mut self) {
        self.curr_index += 1;
        if self.curr_index >= self.tokens.len() {
            panic!("Cannot advance more to make the AST");
        }
        self.curr_tok = self.tokens[self.curr_index].clone();
    }

    // ----------------------------------------------------
    // INFO: Below (until the next block of comment like this)
    // are all the sub-methods for the factor method, which
    // plays a role to make the smallest units of Hackscript (see the grammar description above)
    // It's the smallest unit, but it doesn't mean it needs to be "small". It should act like it's
    // small because if it doesn't behave like that, the code will mess up.
    // ->
    // For example: (1 + 2) * 3 + 4
    // (1 + 2) is clearly not the small unit in this arithmetic expression. But in general
    // it is **indeed** a factor in the (1 + 2) * 3 method, which means it's the smallest unit
    // of the code
    // ----------------------------------------------------
    //
    fn number_making(&mut self) -> Result<AST, Error> {
        let number = AST::new_number(self.curr_tok.clone());
        self.advance();
        Ok(number)
    }
    fn string_making(&mut self) -> Result<AST, Error> {
        let string = AST::new_string(self.curr_tok.clone());
        self.advance();
        Ok(string)
    }

    fn unary_factor_making(&mut self) -> Result<AST, Error> {
        let sign = self.curr_tok.clone();
        let pos_start = self.curr_tok.pos_start.clone();
        self.advance();
        let factor = self.factor()?;
        match factor {
            AST::Number {
                identifier: _,
                value: _,
                pos_start: _,
                pos_end: _,
            }
            | AST::UnaryNumber {
                sign: _,
                value: _,
                pos_start: _,
                pos_end: _,
            }
            | AST::FormingCalc {
                node1: _,
                operator: _,
                node2: _,
                pos_start: _,
                pos_end: _,
            } => Ok(AST::new_unaryfactor(sign, Box::new(factor.clone()))),
            _ => {
                Err(Error::new("OperatorError".to_string(), "Bad operator for the operation (because this operator doesn't technically work for the non-algebraic expression)".to_string(), pos_start, self.curr_tok.pos_end.clone()))
            }
        }
    }
    fn in_parentheses_expr(&mut self) -> Result<AST, Error> {
        let pos_start = self.curr_tok.pos_start.clone();
        self.advance();

        let node1 = self.expr()?;
        let mut vec_tuple: Vec<AST> = vec![node1];
        let mut pass_tuple: bool = false;

        while self.curr_tok._type == COMMA {
            pass_tuple = true;
            self.advance();
            let low = self.expr();
            match low {
                Ok(_) => vec_tuple.push(low?),
                Err(_) => break,
            }
        }
        if self.curr_tok._type != PARENTHESE_CLOSE {
            Err(Error::new(
                "Expect".to_string(),
                "the expression should be closed by a ')' (close parenthese) -> endless expression"
                    .to_string(),
                pos_start,
                self.curr_tok.pos_end.clone(),
            ))
        } else {
            self.advance();
            if self.curr_tok._type == SQUARE_BRACKET_LEFT {
                let index_pos_start = self.curr_tok.pos_start.clone();
                self.advance();
                // check for indexing
                let index = self.expr()?;
                if self.curr_tok._type != SQUARE_BRACKET_RIGHT {
                    Err(Error::new(
                        "Expect".to_string(),
                        "the expression should be closed by a ']' (close parentheses) -> endless expression".to_string(),
                        index_pos_start,
                        self.curr_tok.pos_end.clone(),
                ))
                } else {
                    self.advance();
                    match pass_tuple {
                        false => Ok(AST::new_formingcalc(
                            Box::new(vec_tuple.first().unwrap().clone()),
                            Some(Token::new(
                                String::from(INDEXING),
                                String::new(),
                                index_pos_start,
                                self.curr_tok.pos_end.clone(),
                            )),
                            Box::new(index),
                        )),
                        true => Ok(AST::new_formingcalc(
                            Box::new(AST::new_tuple(
                                vec_tuple,
                                pos_start,
                                index_pos_start.clone(),
                            )),
                            Some(Token::new(
                                String::from(INDEXING),
                                String::new(),
                                index_pos_start,
                                self.curr_tok.pos_end.clone(),
                            )),
                            Box::new(index),
                        )),
                    }
                }
            } else {
                match pass_tuple {
                    false => Ok(vec_tuple.first().unwrap().clone()),
                    true => Ok(AST::new_tuple(
                        vec_tuple,
                        pos_start,
                        self.curr_tok.pos_end.clone(),
                    )),
                }
            }
        }
    }
    fn make_set(&mut self) -> Result<AST, Error> {
        let pos_start = self.curr_tok.pos_start.clone();
        self.advance();

        let node1 = self.expr()?;
        let mut vec_set: Vec<AST> = vec![node1];

        while self.curr_tok._type == COMMA {
            self.advance();
            let low = self.expr();
            match low {
                Ok(_) => vec_set.push(low?),
                Err(_) => break,
            }
        }
        if self.curr_tok._type != CURLY_BRACKET_RIGHT {
            Err(Error::new(
                "Expect".to_string(),
                "the expression should be closed by a ']' (close parenthese) -> endless expression"
                    .to_string(),
                pos_start,
                self.curr_tok.pos_end.clone(),
            ))
        } else {
            self.advance();
            if self.curr_tok._type == SQUARE_BRACKET_LEFT {
                let index_pos_start = self.curr_tok.pos_start.clone();
                self.advance();
                // check for indexing
                let index = self.expr()?;
                if self.curr_tok._type != SQUARE_BRACKET_RIGHT {
                    Err(Error::new(
                        "Expect".to_string(),
                        "the expression should be closed by a ']' (close parentheses) -> endless expression".to_string(),
                        index_pos_start,
                        self.curr_tok.pos_end.clone(),
                ))
                } else {
                    self.advance();
                    Ok(AST::new_formingcalc(
                        Box::new(AST::new_set(vec_set, pos_start, index_pos_start.clone())),
                        Some(Token::new(
                            String::from(INDEXING),
                            String::new(),
                            index_pos_start,
                            self.curr_tok.pos_end.clone(),
                        )),
                        Box::new(index),
                    ))
                }
            } else {
                Ok(AST::new_set(
                    vec_set,
                    pos_start,
                    self.curr_tok.pos_end.clone(),
                ))
            }
        }
    }
    fn make_array(&mut self) -> Result<AST, Error> {
        let pos_start = self.curr_tok.pos_start.clone();
        self.advance();

        let node1 = self.expr()?;
        let mut vec_set: Vec<AST> = vec![node1];

        while self.curr_tok._type == COMMA {
            self.advance();
            let low = self.expr();
            match low {
                Ok(_) => vec_set.push(low?),
                Err(_) => break,
            }
        }
        if self.curr_tok._type != SQUARE_BRACKET_RIGHT {
            Err(Error::new(
                "Expect".to_string(),
                "the expression should be closed by a '}' (close parenthese) -> endless expression"
                    .to_string(),
                pos_start,
                self.curr_tok.pos_end.clone(),
            ))
        } else {
            self.advance();
            if self.curr_tok._type == SQUARE_BRACKET_LEFT {
                let index_pos_start = self.curr_tok.pos_start.clone();
                self.advance();
                // check for indexing
                let index = self.expr()?;
                if self.curr_tok._type != SQUARE_BRACKET_RIGHT {
                    Err(Error::new(
                        "Expect".to_string(),
                        "the expression should be closed by a ']' (close parentheses) -> endless expression".to_string(),
                        index_pos_start,
                        self.curr_tok.pos_end.clone(),
                ))
                } else {
                    self.advance();
                    Ok(AST::new_formingcalc(
                        Box::new(AST::new_array(vec_set, pos_start, index_pos_start.clone())),
                        Some(Token::new(
                            String::from(INDEXING),
                            String::new(),
                            index_pos_start,
                            self.curr_tok.pos_end.clone(),
                        )),
                        Box::new(index),
                    ))
                }
            } else {
                Ok(AST::new_array(
                    vec_set,
                    pos_start,
                    self.curr_tok.pos_end.clone(),
                ))
            }
        }
    }

    fn make_booleans(&mut self) -> Result<AST, Error> {
        let bool = AST::new_boolean(self.curr_tok.clone());
        self.advance();
        Ok(bool)
    }
    fn make_null(&mut self) -> Result<AST, Error> {
        let null = AST::new_null(self.curr_tok.clone());
        self.advance();
        Ok(null)
    }

    // ------------------------------------------------------
    // INFO: THIS IS THE MAIN PART OF THE PARSER
    // ------------------------------------------------------

    fn factor(&mut self) -> Result<AST, Error> {
        if self.curr_tok._type == NUMBER {
            self.number_making()
        } else if self.curr_tok._type == STRING {
            self.string_making()
        } else if [PLUS, MINUS].contains(&self.curr_tok._type.as_str()) {
            self.unary_factor_making()
        } else if self.curr_tok._type == PARENTHESE_OPEN {
            self.in_parentheses_expr()
        } else if [TRUE, FALSE].contains(&self.curr_tok._type.as_str()) {
            self.make_booleans()
        } else if self.curr_tok._type.as_str() == CURLY_BRACKET_LEFT {
            self.make_set()
        } else if self.curr_tok._type.as_str() == SQUARE_BRACKET_LEFT {
            self.make_array()
        } else if self.curr_tok._type.as_str() == NULL {
            self.make_null()
        } else {
            Err(Error::new(
                "Expect".to_string(),
                "a number type token, a string type token, an array, a set, a tuple, null"
                    .to_string(),
                self.curr_tok.pos_start.clone(),
                self.curr_tok.pos_end.clone(),
            ))
        }
    }
    fn bin_op(
        &mut self,
        func: fn(&mut Self) -> Result<AST, Error>,
        list_to_use: Vec<&str>,
    ) -> Result<AST, Error> {
        // Parse the first part
        let node1 = func(self)?;
        let mut high: AST = node1; // Parse the second part

        while list_to_use.contains(&self.curr_tok._type.as_str()) {
            let operator: Option<Token> = Some(self.curr_tok.clone());
            self.advance();
            let low = func(self)?;
            high = AST::new_formingcalc(Box::new(high), operator, Box::new(low));
        }

        let operator: Option<Token> = None;
        Ok(AST::new_formingcalc(
            Box::new(high),
            operator,
            Box::new(AST::new()),
        ))
    }

    fn term(&mut self) -> Result<AST, Error> {
        self.bin_op(Parser::factor, vec![MULTIPLY, DIVIDE])
    }

    fn arithmetic_expr(&mut self) -> Result<AST, Error> {
        self.bin_op(Parser::term, vec![PLUS, MINUS, APPEND])
    }
    fn comp_expr(&mut self) -> Result<AST, Error> {
        self.bin_op(
            Parser::arithmetic_expr,
            vec![
                GREATER,
                LESS,
                GREATER_OR_EQUAL,
                LESS_OR_EQUAL,
                EQUAL,
                NOT_EQUAL,
            ],
        )
    }
    fn expr(&mut self) -> Result<AST, Error> {
        self.bin_op(Parser::comp_expr, vec![AND, OR])
    }
}
