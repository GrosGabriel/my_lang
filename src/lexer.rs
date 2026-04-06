use crate::token::Token;

pub struct Lexer {
    chars: std::iter::Peekable<std::vec::IntoIter<char>>,
}


impl Lexer {
    pub fn new(input: &str) -> Self {

        let chars: Vec<char> = input.chars().collect();

        Self {
            chars: chars.into_iter().peekable(),
        }
    }

    /// Tokenize the entire input
    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while let Some(&ch) = self.chars.peek() {
            match ch {
                ' ' | '\t' => {
                    self.chars.next();
                }
                '0'..='9' => tokens.push(self.read_number()?),
                //'"' => tokens.push(self.read_string()?),
                'A'..='Z'| 'a'..='z' => tokens.push(self.read_word()), 
                '+' => {
                    tokens.push(Token::Plus);
                    self.chars.next();
                }
                '-' => {
                    self.chars.next(); //consume '-'
                    if self.chars.peek() == Some(&'>') {
                        self.chars.next(); //consume '>'
                        tokens.push(Token::Arrow);
                    } else {
                        tokens.push(Token::Minus);
                    }
                }
                '*' => {
                    tokens.push(Token::Star);
                    self.chars.next();
                }
                '/' => {
                    self.chars.next(); //consume '/'
                    if self.chars.peek() == Some(&'/') {
                        // Comment, skip until end of line
                        while let Some(&c) = self.chars.peek() {
                            if c == '\n' { break; }
                            self.chars.next();
                        }
                    } else {
                        tokens.push(Token::Slash);
                    }
                }
                '=' => {
                    self.chars.next(); // consume first '='
                    if self.chars.peek() == Some(&'=') {
                        self.chars.next(); // consomme second '='
                        tokens.push(Token::EqualEqual);
                    } else {
                        tokens.push(Token::Equal);
                    }
                }
                '>' => {
                    tokens.push(Token::Greater);
                    self.chars.next();
                }
                '<' => {
                    tokens.push(Token::Less);
                    self.chars.next();
                }
                '(' => {
                    tokens.push(Token::Lparen);
                    self.chars.next();
                }
                ')' => {
                    tokens.push(Token::Rparen);
                    self.chars.next();
                }

                ':' => {
                    tokens.push(Token::Colon);
                    self.chars.next();
                }
                ',' => {
                    tokens.push(Token::Comma);
                    self.chars.next();
                }
                '[' => {
                    tokens.push(Token::Lbracket);
                    self.chars.next();
                }
                ']' => {
                    tokens.push(Token::Rbracket);
                    self.chars.next();
                }
                _ => {
                    return Err(format!("Unexpected character: '{}'", ch));
                }

            }
        }

        Ok(tokens)
    }

    fn read_number(&mut self) -> Result<Token, String> {
        let mut num = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_ascii_digit() {
                num.push(c);
                self.chars.next();
            } else {
                break;
            }
        }

        num.parse()
            .map(Token::Int)
            .map_err(|_| format!("Invalid number: {}", num))
    }

    // fn read_string(&mut self) -> Result<Token, String> {
    //     self.chars.next(); // skip opening quote
    //     let mut str_val = String::new();

    //     while let Some(c) = self.chars.next() {
    //         if c == '"' {
    //             return Ok(Token::String(str_val));
    //         }
    //         str_val.push(c);
    //     }

    //     Err("Unterminated string".to_string())
    // }

    fn read_word(&mut self) -> Token {
        let mut word = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_alphanumeric() || c == '_' {
                word.push(c);
                self.chars.next();
            } else {
                break;
            }
        }

        match word.as_str() {
            // Keywords
            "let" => Token::Let,
            "in" => Token::In,
            "if" => Token::If,
            "then" => Token::Then,
            "else" => Token::Else,
            "fun" => Token::Fun,
            "fix" => Token::Fix,
            //types
            "Int" => Token::TInt,
            "Bool" => Token::TBool,
            //pairs projections
            "fst" => Token::Fst,
            "snd" => Token::Snd,
            //boolean 
            "true" => Token::True,
            "false" => Token::False,
            "and" => Token::And,
            "or" => Token::Or,
            "not" => Token::Not,
            //list keywords
            "nil" => Token::Nil,
            "cons" => Token::Cons,
            "caselist" => Token::CaseList,
            "reclist" => Token::RecList,
            //sum keywords
            "inl" => Token::Inl,
            "inr" => Token::Inr,
            "casesum" => Token::CaseSum,
            _ => Token::Var(word),
        }
    }
}



