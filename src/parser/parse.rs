use crate::ast::{Declaration, Function, Program, TopLevel};
use crate::lexer::Token;
use crate::parser::expr::{parse_block_items, parse_expr};

pub fn expect(tokens: &[Token], pos: &mut usize, expected: &Token) -> Result<(), String> {
    if tokens.get(*pos) == Some(expected) {
        *pos += 1;
        Ok(())
    } else {
        Err(format!(
            "expected {}, found {}",
            expected,
            tokens
                .get(*pos)
                .map_or_else(|| "EOF".to_string(), std::string::ToString::to_string)
        ))
    }
}

pub fn expect_ident(tokens: &[Token], pos: &mut usize) -> Result<String, String> {
    match tokens.get(*pos) {
        Some(Token::Identifier(name)) => {
            *pos += 1;
            Ok(name.clone())
        }
        other => Err(format!("expected identifier, found {:?}", other)),
    }
}

pub fn parse(tokens: &[Token]) -> Result<Program, String> {
    let mut pos = 0;
    let mut functions = Vec::new();

    while pos < tokens.len() {
        expect(tokens, &mut pos, &Token::KeywordInt)?;
        let name = expect_ident(tokens, &mut pos)?;

        match tokens.get(pos) {
            // global var declaration
            Some(Token::Semicolon) => {
                pos += 1;
                functions.push(TopLevel::GlobalVariable(Declaration::Declare(name, None)));
            }

            // global var declaration and definition
            Some(Token::Equal) => {
                pos += 1;

                let expr = parse_expr(tokens, &mut pos)?;
                expect(tokens, &mut pos, &Token::Semicolon)?;

                functions.push(TopLevel::GlobalVariable(Declaration::Declare(
                    name,
                    Some(expr),
                )));
            }

            // function
            Some(Token::LParen) => {
                pos += 1;

                // parse parameter list: [ "int" <id> { "," "int" <id> } ]
                let mut params = Vec::new();
                if tokens.get(pos) != Some(&Token::RParen) {
                    loop {
                        expect(tokens, &mut pos, &Token::KeywordInt)?;
                        let param = expect_ident(tokens, &mut pos)?;
                        params.push(param);

                        if tokens.get(pos) == Some(&Token::Comma) {
                            pos += 1;
                        } else {
                            break;
                        }
                    }
                }
                expect(tokens, &mut pos, &Token::RParen)?;

                // expect either `{` (definition) or `;` (declaration)
                let body = match tokens.get(pos) {
                    Some(Token::LBrace) => {
                        pos += 1;
                        let mut block_items = Vec::new();
                        while tokens.get(pos) != Some(&Token::RBrace) {
                            let stmts = parse_block_items(tokens, &mut pos)?;
                            block_items.extend(stmts);
                        }
                        expect(tokens, &mut pos, &Token::RBrace)?;
                        Some(block_items)
                    }
                    Some(Token::Semicolon) => {
                        pos += 1;
                        None
                    }
                    other => return Err(format!("Expected '{{' or ';', found {:?}", other)),
                };

                functions.push(TopLevel::Function(Function {
                    name,
                    params,
                    block_items: body,
                }));
            }

            other => return Err(format!("Expected '(' or ';' or '=', found {:?}", other)),
        }
    }

    Ok(Program {
        toplevel_items: functions,
    })
}
