use super::{
    rules::{get_rule, Precedence},
    scanner::TokenType,
    Parser, Result,
};
use crate::{allocate_string, lang_core::chunk::OpCode};
pub(super) fn parse_precedence(parser: &mut Parser, precedence: Precedence) -> Result<()> {
    parser.next();
    let Some(prefix_rule) = get_rule(parser.previous.id).prefix else {
        return parser.error("Expect expression.");
    };
    let can_assign = precedence <= Precedence::Assignment;
    prefix_rule(parser, can_assign)?;
    while precedence <= get_rule(parser.current.id).precedence {
        parser.next();
        let infix_rule = get_rule(parser.previous.id).infix.unwrap();
        infix_rule(parser, can_assign)?;
    }
    Ok(())
}
pub(super) fn binary(parser: &mut Parser, _: bool) -> Result<()> {
    let op_type = parser.previous.id;
    let rule = get_rule(op_type);
    parse_precedence(parser, rule.precedence.add_one())?;

    let op_code = match op_type {
        TokenType::BangEqual | TokenType::GreaterEqual | TokenType::LessEqual => {
            let op = match op_type {
                TokenType::BangEqual => OpCode::Equal,
                TokenType::GreaterEqual => OpCode::Less,
                TokenType::LessEqual => OpCode::Greater,
                _ => unreachable!(),
            };
            parser.emit_bytes(op, OpCode::Not);
            return Ok(());
        }
        TokenType::EqualEqual => OpCode::Equal,
        TokenType::Greater => OpCode::Greater,
        TokenType::Less => OpCode::Less,
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
    parse_precedence(parser, Precedence::Assignment)?;
    Ok(())
}

pub(super) fn number(parser: &mut Parser, _: bool) -> Result<()> {
    let value: i32 = parser.previous.extract().parse().expect("valid number");
    parser.emit_constant(value);
    Ok(())
}
pub(super) fn grouping(parser: &mut Parser, _: bool) -> Result<()> {
    expression(parser)?;
    parser.consume(TokenType::RightParen, "Expect ')' after expression.")
}

pub(super) fn unary(parser: &mut Parser, _: bool) -> Result<()> {
    let operator_id = parser.previous.id;

    // Compile the operand
    parse_precedence(parser, Precedence::Unary)?;

    // Emit the operator instruction.
    let code = match operator_id {
        TokenType::Bang => OpCode::Not,
        TokenType::Minus => OpCode::Negate,
        _ => unreachable!(),
    };
    parser.emit_byte(code);
    Ok(())
}

pub(super) fn literal(parser: &mut Parser, _: bool) -> Result<()> {
    let code = match parser.previous.id {
        TokenType::False => OpCode::False,
        TokenType::True => OpCode::True,
        TokenType::Nil => OpCode::Nil,
        _ => unreachable!(),
    };
    parser.emit_byte(code);
    Ok(())
}

pub(super) fn string(parser: &mut Parser, _: bool) -> Result<()> {
    let string = allocate_string!(parser.previous.extract());
    parser.emit_constant(string);
    Ok(())
}

pub(super) fn variable(parser: &mut Parser, can_assign: bool) -> Result<()> {
    parser.named_variable(parser.previous, can_assign);
    Ok(())
}
pub(super) fn print_statement(parser: &mut Parser) -> Result<()> {
    expression(parser)?;
    parser.consume(TokenType::Semicolon, "Expect ';' after value.")?;
    parser.emit_byte(OpCode::Print);
    Ok(())
}
pub(super) fn expression_statement(parser: &mut Parser) -> Result<()> {
    expression(parser)?;
    parser.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
    parser.emit_byte(OpCode::Pop);
    Ok(())
}
pub(super) fn statement(parser: &mut Parser) -> Result<()> {
    if parser.matches(TokenType::Print) {
        print_statement(parser)
    } else {
        expression_statement(parser)
    }
}
fn parse_variable(parser: &mut Parser, message: &str) -> Result<u8> {
    parser.consume(TokenType::Identifier, message)?;
    Ok(parser.identifier_constant(parser.previous))
}
pub(super) fn var_declaration(parser: &mut Parser) -> Result<()> {
    let global = parse_variable(parser, "Expect variable name.")?;

    if parser.matches(TokenType::Equal) {
        expression(parser)?;
    } else {
        parser.emit_byte(OpCode::Nil);
    }

    parser.consume(
        TokenType::Semicolon,
        "Expect ';' after variable declaration.",
    )?;

    parser.define_variable(global);
    Ok(())
}
pub(super) fn declaration(parser: &mut Parser) -> Result<()> {
    if parser.matches(TokenType::Bind) {
        var_declaration(parser)
    } else {
        statement(parser)
    }
}
