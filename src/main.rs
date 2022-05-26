#[derive(Clone)]
enum Token {
    Num(f64),
    Mul,
    Div,
    Add,
    Sub,
    Lpa,
    Rpa,
    Eol,
}

fn lex(expr: &str) -> Vec<Token> {
    let mut it = expr.chars().peekable();
    let mut tokens = Vec::new();
    while let Some(c) = it.next() {
        match c {
            '0'..='9' => {
                let mut stack = c.to_string();
                while let Some(u) = it.peek() {
                    if !u.is_digit(10) && *u != '.' {
                        break;
                    }
                    stack += &u.to_string();
                    it.next();
                }
                tokens.push(Token::Num(stack.parse().unwrap()));
            }
            '*' => tokens.push(Token::Mul),
            '/' => tokens.push(Token::Div),
            '+' => tokens.push(Token::Add),
            '-' => tokens.push(Token::Sub),
            '(' => tokens.push(Token::Lpa),
            ')' => tokens.push(Token::Rpa),
            _ => (),
        }
    }
    tokens.push(Token::Eol);
    tokens
}

struct Parser {
    iterator: Box<dyn Iterator<Item = Token>>,
    current: Token,
}

struct BinOp {
    left: Box<dyn Node>,
    op: Token,
    right: Box<dyn Node>,
}

struct UnaryOp {
    op: Token,
    val: Box<dyn Node>,
}

struct Num {
    val: f64,
}

trait Node {
    fn resolve(&self) -> f64;
}

impl Node for BinOp {
    fn resolve(&self) -> f64 {
        match self.op {
            Token::Add => self.left.resolve() + self.right.resolve(),
            Token::Sub => self.left.resolve() - self.right.resolve(),
            Token::Mul => self.left.resolve() * self.right.resolve(),
            Token::Div => self.left.resolve() / self.right.resolve(),
            _ => 0f64,
        }
    }
}

impl Node for UnaryOp {
    fn resolve(&self) -> f64 {
        match self.op {
            Token::Add => self.val.resolve(),
            Token::Sub => -self.val.resolve(),
            _ => 0f64,
        }
    }
}

impl Node for Num {
    fn resolve(&self) -> f64 {
        self.val
    }
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        let mut it = tokens.into_iter();
        let cur = it.next().unwrap();
        Parser {
            iterator: Box::new(it),
            current: cur,
        }
    }

    fn next(&mut self) -> Token {
        self.current = self.iterator.next().unwrap();
        self.current.clone()
    }

    fn factor(&mut self) -> Box<dyn Node> {
        let t = self.current.clone();
        match t {
            Token::Num(x) => {
                self.next();
                return Box::new(Num { val: x });
            }
            Token::Add | Token::Sub => {
                self.next();
                return Box::new(UnaryOp {
                    op: t,
                    val: self.factor(),
                });
            }
            Token::Lpa => {
                self.next();
                let node = self.expr();
                self.next();
                return node;
            }
            _ => (),
        }
        return Box::new(Num { val: 0f64 });
    }

    fn term(&mut self) -> Box<dyn Node> {
        let mut node = self.factor();

        loop {
            let t = self.current.clone();
            match t {
                Token::Mul | Token::Div => {
                    self.next();
                    node = Box::new(BinOp {
                        left: node,
                        op: t,
                        right: self.factor(),
                    });
                }
                _ => break,
            }
        }

        return node;
    }

    fn expr(&mut self) -> Box<dyn Node> {
        let mut node = self.term();

        loop {
            let t = self.current.clone();
            match t {
                Token::Add | Token::Sub => {
                    self.next();
                    node = Box::new(BinOp {
                        left: node,
                        op: t,
                        right: self.term(),
                    });
                }
                _ => break,
            }
        }

        return node;
    }
}

fn calc(expr: &str) -> f64 {
    let tokens = lex(expr);
    let mut parser = Parser::new(tokens);
    let ast = parser.expr();
    return ast.resolve();
}

fn main() {
    let expr = "((2.33 / (2.9+3.5)*4) - -6)";
    println!("{}={}", expr, calc(expr));
}
