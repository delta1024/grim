use super::{functions::*, scanner::TokenType, Parser, Result};

pub(super) type ParseFn = fn(&mut Parser) -> Result<()>;

pub(super) struct ParseRule {
    pub(super) prefix: Option<ParseFn>,
    pub(super) infix: Option<ParseFn>,
    pub(super) precedence: Precedence,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Precedence {
    None,
    /// =
    Assignment,
    /// ||
    Or,
    /// &&
    And,
    /// == !=
    Equality,
    /// < > <= >=
    Comparison,
    /// + -
    Term,
    /// * /
    Factor,
    /// ! -
    Unary,
    /// . ()
    Call,
    Primary,
    Overflow,
}

impl Precedence {
    pub(super) fn add_one(&self) -> Self {
        match self {
            Self::None => Self::Assignment,
            Self::Assignment => Self::Or,
            Self::Or => Self::And,
            Self::And => Self::Equality,
            Self::Equality => Self::Comparison,
            Self::Comparison => Self::Term,
            Self::Term => Self::Factor,
            Self::Factor => Self::Unary,
            Self::Unary => Self::Call,
            Self::Call => Self::Primary,
            Self::Primary => Self::Overflow,
            Self::Overflow => unreachable!(),
        }
    }
}

macro_rules! define {
    ($_: expr, $prefix: expr, $infix: expr, $precedence: expr  ) => {
        ParseRule {
            prefix: $prefix,
            infix: $infix,
            precedence: $precedence,
        }
    };
}

pub(super) fn get_rule(id: TokenType) -> &'static ParseRule {
    &RULES[id as usize]
}

#[rustfmt::skip]
const RULES: [ParseRule; 40] = [
    define!{LeftParen   , Some(grouping), None        , Precedence::None       },
    define!{RightParen  , None          , None        , Precedence::None       },
    define!{LeftBrace   , None          , None        , Precedence::None       },
    define!{RightBrace  , None          , None        , Precedence::None       },
    define!{RightBracket, None          , None        , Precedence::None       },
    define!{LeftBracket , None          , None        , Precedence::None       },
    define!{Plus        , None          , Some(binary), Precedence::Term       },
    define!{Star        , None          , Some(binary), Precedence::Factor     },
    define!{Slash       , None          , Some(binary), Precedence::Factor     },
    define!{Comma       , None          , None        , Precedence::None       },
    define!{Semicolon   , None          , None        , Precedence::None       },
    define!{Equal       , None          , None        , Precedence::None       },
    define!{EqualEqual  , None          , Some(binary), Precedence::Equality   },
    define!{Less        , None          , Some(binary), Precedence::Comparison },
    define!{LessEqual   , None          , Some(binary), Precedence::Comparison },
    define!{Greater     , None          , Some(binary), Precedence::Comparison },
    define!{GreaterEqual, None          , Some(binary), Precedence::Comparison },
    define!{Bang        , Some(unary)   , None        , Precedence::None       },
    define!{BangEqual   , None          , Some(binary), Precedence::Equality   },
    define!{Dot         , None          , None        , Precedence::None       },
    define!{DotDot      , None          , None        , Precedence::None       },
    define!{Minus       , Some(unary)   , Some(binary), Precedence::Term       },
    define!{MinusColon  , None          , None        , Precedence::None       },
    define!{OrOr        , None          , None        , Precedence::None       },
    define!{AndAnd      , None          , None        , Precedence::None       },
    define!{Number      , Some(number)  , None        , Precedence::None       },
    define!{String      , None          , None        , Precedence::None       },
    define!{Identifier  , None          , None        , Precedence::None       },
    define!{CharLit     , None          , None        , Precedence::None       },
    define!{True        , Some(literal) , None        , Precedence::None       },
    define!{False       , Some(literal) , None        , Precedence::None       },
    define!{Struct      , None          , None        , Precedence::None       },
    define!{Enum        , None          , None        , Precedence::None       },
    define!{Char        , None          , None        , Precedence::None       },
    define!{Int         , None          , None        , Precedence::None       },
    define!{Nil         , Some(literal) , None        , Precedence::None       },
    define!{Typedef     , None          , None        , Precedence::None       },
    define!{Bind        , None          , None        , Precedence::None       },
    define!{Def         , None          , None        , Precedence::None       },
    define!{EOF         , None          , None        , Precedence::None       },
];
