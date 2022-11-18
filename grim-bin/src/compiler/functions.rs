use super::{
    rules::{get_rule, Precedence},
    scanner::TokenType,
    Parser, Result,
};
use crate::core::chunk::OpCode;
pub(super) fn parse_precedence(parser: &mut Parser, precedence: Precedence) -> Result<()> {
    parser.next();
    dbg!(parser.previous.id);
    let Some(prefix_rule) = get_rule(parser.previous.id).prefix else {
        return parser.error("Expect expression.");
    };
    prefix_rule(parser)?;
    while precedence <= get_rule(parser.current.id).precedence {
        parser.next();
        let infix_rule = get_rule(parser.previous.id).infix.unwrap();
        infix_rule(parser)?;
    }
    Ok(())
}
pub(super) fn binary(parser: &mut Parser) -> Result<()> {
    let op_type = parser.previous.id;
    let rule = get_rule(op_type);
    parse_precedence(parser, rule.precedence.add_one())?;

    let op_code = match op_type {
        TokenType::Plus => OpCode::Add,
        TokenType::Minus => OpCode::Subtract,
        TokenType::Star => OpCode::Multiply,
        TokenType::Slash => OpCode::Divide,
        _ => unreachable!(),
    };
    parser.emit_byte(op_code);
    Ok(())
}
pub(super) fn expression(parser: &mut Parser) -> Result<()> {
    parse_precedence(parser, Precedence::Assignment)
}

pub(super) fn number(parser: &mut Parser) -> Result<()> {
    let value: i32 = parser.previous.extract().parse().expect("valid number");
    parser.emit_constant(value);
    Ok(())
}
pub(super) fn grouping(parser: &mut Parser) -> Result<()> {
    expression(parser)?;
    parser.consume(TokenType::RightParen, "Expect ')' after expression.")
}

pub(super) fn unary(parser: &mut Parser) -> Result<()> {
    let operator_id = parser.previous.id;

    // Compile the operand
    parse_precedence(parser, Precedence::Unary)?;

    // Emit the operator instruction.
    match operator_id {
        TokenType::Minus => parser.emit_byte(OpCode::Negate),
        _ => unreachable!(),
    }
    Ok(())
}
