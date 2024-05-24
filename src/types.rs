#[derive(Debug, PartialEq)]
pub enum Error {
    ConsistentScopeError(String),
}

/// Struct representing a token generated by the lexer
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenType,
    pub value: Option<RepData>,
}

/// Sub-struct of `TOKEN` that stores the kind of Token
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    PRINT,
    STRING,
    FUNCTION,
    RPAREN,
    LPAREN,
    RETURN,
    NULL,
    NUMBER,
    BINOP(InfixOperation),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expr(pub RepData, pub InfixOperation, pub RepData);

/// Allowed binary, infix operations
#[derive(Debug, Clone, PartialEq)]
pub enum InfixOperation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

/// Represented Data: Sub-struct of `TOKEN` that stores values depending on the `TOKENTYPE`
#[derive(Debug, Clone, PartialEq)]
pub enum RepData {
    STRING(String),
    NUMBER(Number),
    TWONUMBER(Number, Number),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    UINT(usize),
    IINT(isize),
}

impl Token {
    pub fn as_string(&self) -> String {
        match &self.kind {
            TokenType::PRINT => "PRINT".to_owned(),
            TokenType::STRING => format!("{:?}", self.value),
            TokenType::FUNCTION => "FUNCTION".to_owned(),
            TokenType::RPAREN => "RPAREN".to_owned(),
            TokenType::LPAREN => "LPAREN".to_owned(),
            TokenType::RETURN => format!("{:?}", self.value),
            TokenType::NULL => "NULL".to_owned(),
            TokenType::BINOP(op) => format!("{:?}", op),
            TokenType::NUMBER => format!("NUMBER({:?})", self.value),
        }
    }
}

impl From<TokenType> for Token {
    fn from(value: TokenType) -> Self {
        let self_default_value = match &value {
            TokenType::PRINT => Some(RepData::STRING("".to_string())),
            TokenType::STRING => Some(RepData::STRING("".to_string())),
            _ => None,
        };

        Token {
            kind: value,
            value: self_default_value,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::RepData;

    use super::{Token, TokenType};

    #[test]
    fn check_token_into() {
        let tokentype_1 = TokenType::LPAREN;
        let tokentype_2 = TokenType::STRING;
        let expected_token_1 = Token {
            kind: TokenType::LPAREN,
            value: None,
        };
        let expected_token_2 = Token {
            kind: TokenType::STRING,
            value: Some(RepData::STRING("".to_string())),
        };

        assert_eq!(expected_token_1, tokentype_1.into());
        assert_eq!(expected_token_2, tokentype_2.into());
    }
}
