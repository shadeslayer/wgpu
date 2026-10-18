use super::*;

pub(super) fn parenthesized_declaration(
    parser: &Parser<'_>,
    mut lexer: Lexer<'_>,
) -> Result<bool, ParseError> {
    if !declarator(parser, &mut lexer, 0)? {
        return Ok(false);
    }
    if lexer.peek()?.kind == TokenKind::Symbol('(') && !balanced(&mut lexer, 0)? {
        return Ok(false);
    }
    Ok(matches!(
        lexer.peek()?.kind,
        TokenKind::Symbol(';' | '=' | ',' | '{')
    ))
}

fn declarator(parser: &Parser<'_>, lexer: &mut Lexer<'_>, depth: u32) -> Result<bool, ParseError> {
    check_depth(lexer, depth)?;
    while matches!(
        lexer.peek()?.kind,
        TokenKind::Symbol('*' | '&') | TokenKind::Operator("&&")
    ) {
        lexer.next()?;
        lexer.next_if(TokenKind::Ident("const"))?;
    }
    match lexer.peek()?.kind {
        TokenKind::Ident(name) if !Parser::reserved_name(name) => {
            lexer.next()?;
        }
        TokenKind::Symbol('(') => {
            lexer.next()?;
            if !declarator(parser, lexer, depth + 1)? || !lexer.next_if(TokenKind::Symbol(')'))? {
                return Ok(false);
            }
        }
        _ => return Ok(false),
    }
    loop {
        match lexer.peek()?.kind {
            TokenKind::Symbol('[') => {
                if !balanced(lexer, depth + 1)? {
                    return Ok(false);
                }
            }
            TokenKind::Symbol('(') => {
                let checkpoint = lexer.clone();
                if !parameters(parser, lexer, depth + 1)? {
                    *lexer = checkpoint;
                    break;
                }
            }
            _ => break,
        }
    }
    Ok(true)
}

fn parameters(parser: &Parser<'_>, lexer: &mut Lexer<'_>, depth: u32) -> Result<bool, ParseError> {
    check_depth(lexer, depth)?;
    lexer.next()?;
    if lexer.next_if(TokenKind::Symbol(')'))? {
        return Ok(true);
    }
    loop {
        if !lexer.next_if(TokenKind::Ident("void"))? {
            match lexer.peek()?.kind {
                TokenKind::Ident("const") => {}
                TokenKind::Ident(name) if parser.type_name(name).is_some() => {}
                _ => return Ok(false),
            }
            parser.parse_type_from(lexer)?;
        }
        if !matches!(lexer.peek()?.kind, TokenKind::Symbol(',' | ')' | '='))
            && !declarator(parser, lexer, depth + 1)?
        {
            return Ok(false);
        }
        if lexer.next_if(TokenKind::Symbol('='))? {
            while !matches!(lexer.peek()?.kind, TokenKind::Symbol(',' | ')')) {
                match lexer.peek()?.kind {
                    TokenKind::Symbol('(' | '[' | '{') => {
                        if !balanced(lexer, depth + 1)? {
                            return Ok(false);
                        }
                    }
                    TokenKind::End => return Ok(false),
                    _ => {
                        lexer.next()?;
                    }
                }
            }
        }
        if lexer.next_if(TokenKind::Symbol(')'))? {
            return Ok(true);
        }
        if !lexer.next_if(TokenKind::Symbol(','))? {
            return Ok(false);
        }
    }
}

fn balanced(lexer: &mut Lexer<'_>, depth: u32) -> Result<bool, ParseError> {
    check_depth(lexer, depth)?;
    let closing = match lexer.next()?.kind {
        TokenKind::Symbol('(') => ')',
        TokenKind::Symbol('[') => ']',
        TokenKind::Symbol('{') => '}',
        _ => unreachable!(),
    };
    loop {
        match lexer.peek()?.kind {
            TokenKind::Symbol('(' | '[' | '{') => {
                if !balanced(lexer, depth + 1)? {
                    return Ok(false);
                }
            }
            TokenKind::Symbol(')' | ']' | '}') => {
                return lexer.next_if(TokenKind::Symbol(closing));
            }
            TokenKind::End => return Ok(false),
            _ => {
                lexer.next()?;
            }
        }
    }
}

fn check_depth(lexer: &mut Lexer<'_>, depth: u32) -> Result<(), ParseError> {
    if depth > 64 {
        Err(ParseError {
            message: String::from("CUDA declaration nesting limit exceeded"),
            span: lexer.peek()?.span,
        })
    } else {
        Ok(())
    }
}
