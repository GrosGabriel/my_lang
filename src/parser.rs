use crate::token::Token;
use crate::term::Term;
use crate::typechecker::Type;


pub struct Parser {
    tokens : Vec<Token>,
    pos : usize, //actual position 
}



impl Parser {

    pub fn new(tokens : Vec<Token>) -> Self {
        Self {
            tokens,
            pos : 0,
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.pos += 1;
        }
    }

    fn consume_simple(&mut self, expected: Token, msg: &str) -> Result<(), String> {
        if let Some(tok) = self.peek() {
            if std::mem::discriminant(tok) == std::mem::discriminant(&expected) {
                self.advance();
                return Ok(());
            }
        }
        Err(msg.to_string())
    }

    fn parse_var_name(&mut self) -> Result<String, String> {
        match self.peek() {
            Some(Token::Var(name)) => {
                let name = name.clone();
                self.advance();
                Ok(name)
            }
            _ => Err("Expected variable name".to_string()),
        }
    }

    fn can_start_atom(&self) -> bool {
        match self.peek() {
            Some(Token::Int(_)) => true,
            Some(Token::Var(_)) => true,
            Some(Token::True) => true,
            Some(Token::False) => true,
            Some(Token::Lparen) => true,
            Some(Token::Fst) => true,
            Some(Token::Snd) => true,
            Some(Token::Nil) => true,
            Some(Token::Cons) => true,
            Some(Token::CaseList) => true,
            Some(Token::RecList) => true,
            Some(Token::Inl) => true,
            Some(Token::Inr) => true,
            Some(Token::CaseSum) => true,
            _ => false,
        }
    }

    pub fn parse(&mut self) -> Result<Term, String> {
        let expr = self.parse_expr()?;
        if !self.is_at_end() {
            return Err(format!("Unexpected token at position {}", self.pos));
        }
        Ok(expr)
    }

    fn parse_expr(&mut self) -> Result<Term, String> {
        match self.peek() {
            Some(Token::Let) => self.parse_let(),
            Some(Token::If) => self.parse_if(),
            Some(Token::Fun) => self.parse_fun(),
            _ => self.parse_equality(),
        }
    }

    fn parse_let(&mut self) -> Result<Term, String> {
        self.consume_simple(Token::Let, "Expected 'let'")?;
        let name = self.parse_var_name()?;
        self.consume_simple(Token::Colon, "Expected ':' after variable name")?;
        let ty = self.parse_type()?;
        self.consume_simple(Token::Equal, "Expected '='")?;
        let val = self.parse_expr()?;
        self.consume_simple(Token::In, "Expected 'in'")?;
        let body = self.parse_expr()?;
        Ok(Term::Let {
            name,
            ty,
            val: Box::new(val),
            body: Box::new(body),
        })
    }

    fn parse_if(&mut self) -> Result<Term, String> {
        self.consume_simple(Token::If, "Expected 'if'")?;
        let cond = self.parse_expr()?;
        self.consume_simple(Token::Then, "Expected 'then'")?;
        let if_true = self.parse_expr()?;
        self.consume_simple(Token::Else, "Expected 'else'")?;
        let if_false = self.parse_expr()?;
        Ok(Term::Ite {
            cond: Box::new(cond),
            if_true: Box::new(if_true),
            if_false: Box::new(if_false),
        })
    }

    // Parse a base type (Int, Bool, or parenthesized type)
    fn parse_base_type(&mut self) -> Result<Type, String> {
        match self.peek() {
            Some(Token::TInt) => {
                self.advance();
                Ok(Type::Int)
            }
            Some(Token::TBool) => {
                self.advance();
                Ok(Type::Bool)
            }

            Some(Token::Lbracket) => {
                self.advance();
                let ty = self.parse_type()?;
                self.consume_simple(Token::Rbracket, "Expected ']'")?;
                Ok(Type::List(Box::new(ty)))
            }

            Some(Token::Lparen) => {
                self.advance();
                let ty = self.parse_type_arrow_or_sum_or_pair()?;  // Arrow authorized between parentheses
                self.consume_simple(Token::Rparen, "Expected ')' after type")?;
                Ok(ty)
            }
            _ => Err(format!("Expected type at position {}", self.pos)),
        }
    }

    // Parse an arrow type A -> B -- only called between parentheses ; 
    // or a sum type A + B -- only called between parentheses
    // or a pair type A, B -- only called between parentheses
    fn parse_type_arrow_or_sum_or_pair(&mut self) -> Result<Type, String> {
        let left = self.parse_base_type()?;
        if matches!(self.peek(), Some(Token::Arrow)) {
            self.advance();
            let right = self.parse_type_arrow_or_sum_or_pair()?;  // recursive for (A -> (B -> C))
            return Ok(Type::Arrow(Box::new(left), Box::new(right)));
        }
        if matches!(self.peek(), Some(Token::Plus)) {
            self.advance();
            let right = self.parse_type_arrow_or_sum_or_pair()?;  // recursive for (A + (B + C))
            return Ok(Type::Sum(Box::new(left), Box::new(right)));
        }
        if matches!(self.peek(), Some(Token::Comma)) {
            self.advance();
            let right = self.parse_type_arrow_or_sum_or_pair()?; // recursive for (A, (B, C))
            return Ok(Type::Pair(Box::new(left), Box::new(right)));
        }
        Ok(left)
    }

    // parsing a type
    fn parse_type(&mut self) -> Result<Type, String> {
        self.parse_base_type()
    }

    // parse_fun mis à jour
    fn parse_fun(&mut self) -> Result<Term, String> {
        self.consume_simple(Token::Fun, "Expected 'fun'")?;
        let param = self.parse_var_name()?;
        self.consume_simple(Token::Colon, "Expected ':' after parameter name")?;
        let ty = self.parse_type()?;
        self.consume_simple(Token::Arrow, "Expected '->'")?;
        let body = self.parse_expr()?;
        Ok(Term::Abs {
            var: param,
            ty,
            body: Box::new(body),
        })
    }

    fn parse_equality(&mut self) -> Result<Term, String> {
        let mut left = self.parse_comparison()?;
        while let Some(Token::EqualEqual) = self.peek() {
            self.advance();
            let right = self.parse_comparison()?;
            left = Term::Eq(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Term, String> {
        let mut left = self.parse_or()?;
        loop {
            match self.peek() {
                Some(Token::Greater) => {
                    self.advance();
                    let right = self.parse_or()?;
                    left = Term::Greater(Box::new(left), Box::new(right));
                }
                Some(Token::Less) => {
                    self.advance();
                    let right = self.parse_or()?;
                    left = Term::Less(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }


    fn parse_or(&mut self) -> Result<Term, String> {
        let mut left = self.parse_and()?;
        while let Some(Token::Or) = self.peek() {
            self.advance();
            let right = self.parse_and()?;
            left = Term::Or(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Term, String> {
        let mut left = self.parse_not()?;
        while let Some(Token::And) = self.peek() {
            self.advance();
            let right = self.parse_not()?;
            left = Term::And(Box::new(left), Box::new(right));
        }
        Ok(left)
    }


    fn parse_not(&mut self) -> Result<Term, String> {
        if let Some(Token::Not) = self.peek() {
            self.advance();
            let inner = self.parse_not()?;  // récursif : not not x est valide
            Ok(Term::Not(Box::new(inner)))
        } else {
            self.parse_add_sub()  // ← reprend la chaîne existante
        }
    }

    fn parse_add_sub(&mut self) -> Result<Term, String> {
        let mut left = self.parse_mul_div()?;
        loop {
            match self.peek() {
                Some(Token::Plus) => {
                    self.advance();
                    let right = self.parse_mul_div()?;
                    left = Term::Add(Box::new(left), Box::new(right));
                }
                Some(Token::Minus) => {
                    self.advance();
                    let right = self.parse_mul_div()?;
                    left = Term::Sub(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_mul_div(&mut self) -> Result<Term, String> {
        let mut left = self.parse_application()?;
        loop {
            match self.peek() {
                Some(Token::Star) => {
                    self.advance();
                    let right = self.parse_application()?;
                    left = Term::Mul(Box::new(left), Box::new(right));
                }
                Some(Token::Slash) => {
                    self.advance();
                    let right = self.parse_application()?;
                    left = Term::Div(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_application(&mut self) -> Result<Term, String> {
        let mut term = self.parse_unary()?;
        while self.can_start_atom() || matches!(self.peek(), Some(Token::Fix)) {
            let arg = self.parse_unary()?;
            term = Term::App(Box::new(term), Box::new(arg));
        }
        Ok(term)
    }

    fn parse_unary(&mut self) -> Result<Term, String> {
        if let Some(Token::Fix) = self.peek() {
            self.advance();
            let inner = self.parse_unary()?;
            Ok(Term::Fix(Box::new(inner)))
        } else if let Some(Token::Minus) = self.peek() {
            self.advance();
            match self.peek() {
                Some(Token::Int(n)) => {
                    let n = -*n;
                    self.advance();
                    Ok(Term::Int(n))
                }
                Some(Token::Lparen) => {
                    let inner = self.parse_atom()?;
                    Ok(Term::Sub(Box::new(Term::Int(0)), Box::new(inner)))
                }
            
                _ => Err("Expected number or '(' after unary '-'".to_string()),
            }
        } else {  
            self.parse_atom()
        }
    }

    fn parse_atom(&mut self) -> Result<Term, String> {
        match self.peek() {
            Some(Token::Int(n)) => {
                let v = *n;
                self.advance();
                Ok(Term::Int(v))
            }
            Some(Token::Var(name)) => {
                let name = name.clone();
                self.advance();
                Ok(Term::Var(name))
            }
            Some(Token::True) => {
                self.advance();
                Ok(Term::True)
            }
            Some(Token::False) => {
                self.advance();
                Ok(Term::False)
            }
            Some(Token::Lparen) => {
                self.advance();
                let first = self.parse_expr()?;
                match self.peek() {
                    Some(Token::Comma) => {
                        self.advance();
                        let second = self.parse_expr()?;
                        self.consume_simple(Token::Rparen, "Expected ')' after pair")?;
                        Ok(Term::Pair(Box::new(first), Box::new(second)))
                    }
                    _ => {
                        self.consume_simple(Token::Rparen, "Expected ')' after expression")?;
                        Ok(first)
                    }
                }
            }
            Some(Token::Fst) => {
                self.advance();
                let pair = self.parse_atom()?;
                Ok(Term::Fst(Box::new(pair)))
            }
            Some(Token::Snd) => {
                self.advance();
                let pair = self.parse_atom()?;
                Ok(Term::Snd(Box::new(pair)))
            }
            Some(Token::Nil) => {
                self.advance();
                self.consume_simple(Token::Lbracket, "Expected '[' after 'nil'")?;
                let inner_ty = self.parse_type()?;
                self.consume_simple(Token::Rbracket, "Expected ']'")?;
                Ok(Term::Nil(inner_ty))
            }
            Some(Token::Cons) => {
                self.advance();
                let head = self.parse_atom()?;
                let tail = self.parse_atom()?;
                Ok(Term::Cons(Box::new(head), Box::new(tail)))
            }
            Some(Token::CaseList) => {
                self.advance();
                let scrutinee = self.parse_atom()?;
                let if_nil = self.parse_atom()?;
                let if_cons = self.parse_atom()?;
                Ok(Term::CaseList { 
                    scrutinee: Box::new(scrutinee), 
                    if_nil: Box::new(if_nil), 
                    if_cons: Box::new(if_cons) 
                })
            }
            Some(Token::RecList) => {
                self.advance();
                let scrutinee = self.parse_atom()?;
                let if_nil = self.parse_atom()?;
                let if_cons = self.parse_atom()?;
                Ok(Term::RecList { 
                    scrutinee: Box::new(scrutinee), 
                    if_nil: Box::new(if_nil), 
                    if_cons: Box::new(if_cons) 
                })
            }

            Some(Token::Inl) => {
                self.advance();
                let t  = self.parse_atom()?;
                self.consume_simple(Token::Lparen, "Expected '(' after 'inl' value")?;
                let r_ty = self.parse_type()?;
                self.consume_simple(Token::Rparen, "Expected ')'")?;
                Ok(Term::Inl { t: Box::new(t), r_ty})
            }

            Some(Token::Inr) => {
                self.advance();
                let t  = self.parse_atom()?;
                self.consume_simple(Token::Lparen, "Expected '(' after 'inr' value")?;
                let l_ty = self.parse_type()?;
                self.consume_simple(Token::Rparen, "Expected ')'")?;
                Ok(Term::Inr { t: Box::new(t), l_ty})
            }

            Some(Token::CaseSum) => {
                self.advance();
                let scrutinee = self.parse_atom()?;
                let inl_case = self.parse_atom()?;
                let inr_case = self.parse_atom()?;
                Ok(Term::CaseSum { 
                    scrutinee: Box::new(scrutinee), 
                    inl_case: Box::new(inl_case), 
                    inr_case: Box::new(inr_case) 
                })
            }

            _ => Err(format!("Expected expression at token position {}", self.pos)),
        }
    }



    pub fn parse_decl(&mut self) -> Result<(String, Type, Term), String> {
        self.consume_simple(Token::Let, "Expected 'let'")?;
        let name = self.parse_var_name()?;
        self.consume_simple(Token::Colon, "Expected ':'")?;
        let ty = self.parse_type()?;
        self.consume_simple(Token::Equal, "Expected '='")?;
        let val = self.parse_expr()?;

        if !self.is_at_end() {
            return Err(format!("Unexpected token at position {} after declaration", self.pos));
        }
        Ok((name, ty, val))
    }
}

