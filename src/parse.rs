use crate::lex::{Operator, Token, TokenStream};

#[derive(Debug)]
struct FunctionPointer;

#[derive(Debug)]
struct FunctionInvocation {
    func: FunctionPointer,
    parameters: Vec<Expression>,
}

#[derive(Debug)]
enum ValueType {
    Number(i32),
}

#[derive(Debug)]
enum Expression {
    Singleton(ValueType),
    Invocation(FunctionInvocation),
    Operation(Box<Expression>, Operator, Box<Expression>),
}

#[derive(Clone, Debug)]
pub enum ParseError {
    Unexpected,
}

type PResult<T> = Result<T, ParseError>;

pub struct Parser {
    toks: Vec<Token>,
    index: usize,
}

impl Parser {
    fn next(&mut self) -> Option<&Token> {
        if self.index >= self.toks.len() {
            None
        } else {
            let tok = &self.toks[self.index];
            self.index += 1;
            Some(tok)
        }
    }

    fn peek(&mut self) -> Option<&Token> {
        self.npeek(0)
    }

    fn npeek(&mut self, n: usize) -> Option<&Token> {
        if self.index + n >= self.toks.len() {
            None
        } else {
            Some(&self.toks[self.index + n])
        }
    }

    pub fn new(tokstrm: TokenStream) -> Self {
        let toks = tokstrm.collect();

        Self { toks, index: 0 }
    }

    pub fn parse(&mut self) -> PResult<()> {
        let expr = self.try_parse_expression();

        println!("{:?}", expr);

        Ok(())
    }

    fn peek_operator(&mut self) -> bool {
        match self.peek() {
            None => {
                println!("Matched none");
                return false;
            }
            Some(tok) => match tok {
                Token::Operator(_) => {
                    println!("Matched op");
                    return true;
                }
                _ => {
                    return false;
                }
            },
        }
    }

    fn try_parse_valtype(&mut self) -> PResult<ValueType> {
        let t = self.peek();

        if t.is_some() {
            let t = t.unwrap();
            println!("{:?}", t);
            if t.is_int_literal() {
                let t = self.next();
                println!("{:?}", t);
                match t.unwrap() {
                    Token::IntLiteral(val) => Ok(ValueType::Number(*val)),
                    _ => Err(ParseError::Unexpected),
                }
            } else {
                Err(ParseError::Unexpected)
            }
        } else {
            Err(ParseError::Unexpected)
        }
    }

    fn try_parse_expression(&mut self) -> PResult<Expression> {
        // expr (op) expr | (expr) | valtype | funccall

        {
            let valtype = self.try_parse_valtype();

            println!("{:?}", valtype);

            if valtype.is_ok() {
                println!("Read valtype");
                if self.peek_operator() {
                    match self.next().unwrap() {
                        Token::Operator(op) => {
                            println!("Parsing operation");
                            let owned_op = op.clone();
                            let rhs = self.try_parse_expression();
                            let lhs = Expression::Singleton(valtype.unwrap());

                            if rhs.is_ok() {
                                return Ok(Expression::Operation(
                                    Box::new(lhs),
                                    owned_op,
                                    Box::new(rhs.unwrap()),
                                ));
                            } else {
                                return Err(ParseError::Unexpected);
                            }
                        }
                        _ => return Err(ParseError::Unexpected),
                    }
                } else {
                    return Ok(Expression::Singleton(valtype.unwrap()));
                }
            }

            unimplemented!()
        }

        /*let func_call = self.try_parse_func_call();
        if func_call.is_ok() {


        } else {
        }*/
    }
}
