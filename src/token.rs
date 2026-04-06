#[derive(Debug, Clone)]
pub enum Token {
    Int(i64),
    Var(String),

    //Booleans
    True,
    False,
    And,
    Or,
    Not,
    

    // Keywords
    Let,
    In,
    Fun,
    Arrow,
    Fix,
    If,
    Then,
    Else,


    // Operators
    Plus,
    Minus,
    Star,
    Slash,

    Equal,
    EqualEqual,
    Greater,
    Less,

    Lparen,
    Rparen,

    // Types
    Colon, // : 
    TInt, // Type Int, noted Int in the parser
    TBool, // Type Bool noted Bool in the parser

    Comma, // , used for pairs
    Fst, // first projection of a pair
    Snd, // second projection of a pair


    // List 
    Lbracket, // [
    Rbracket, // ]
    Nil, // empty list
    Cons,
    CaseList,
    RecList,

    // Sum
    Inl,
    Inr,
    CaseSum,
}
