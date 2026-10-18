use super::*;

pub(super) fn parenthesized_declaration(
    parser: &Parser<'_>,
    mut lexer: Lexer<'_>,
) -> Result<bool, ParseError> {
    if !declarator(parser, &mut lexer, 0)? {
        return Ok(false);
    }
    if lexer.peek()?.0 == Token::Paren('(') && !balanced(&mut lexer, 0)? {
        return Ok(false);
    }
    Ok(matches!(
        lexer.peek()?.0,
        Token::Separator(';') | Token::Operation('=') | Token::Separator(',') | Token::Paren('{')
    ))
}

fn declarator(parser: &Parser<'_>, lexer: &mut Lexer<'_>, depth: u32) -> Result<bool, ParseError> {
    check_depth(lexer, depth)?;
    while matches!(
        lexer.peek()?.0,
        Token::Operation('*' | '&') | Token::LogicalOperation('&')
    ) {
        lexer.next()?;
        lexer.next_if(Token::Word("const"))?;
    }
    match lexer.peek()?.0 {
        Token::Word(name) if !Parser::reserved_name(name) => {
            lexer.next()?;
        }
        Token::Paren('(') => {
            lexer.next()?;
            if !declarator(parser, lexer, depth + 1)? || !lexer.next_if(Token::Paren(')'))? {
                return Ok(false);
            }
        }
        _ => return Ok(false),
    }
    loop {
        match lexer.peek()?.0 {
            Token::Paren('[') => {
                if !balanced(lexer, depth + 1)? {
                    return Ok(false);
                }
            }
            Token::Paren('(') => {
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
    if lexer.next_if(Token::Paren(')'))? {
        return Ok(true);
    }
    loop {
        if !lexer.next_if(Token::Word("void"))? {
            match lexer.peek()?.0 {
                Token::Word("const") => {}
                Token::Word(name) if parser.type_name(name).is_some() => {}
                _ => return Ok(false),
            }
            parser.parse_type_from(lexer)?;
        }
        if !matches!(
            lexer.peek()?.0,
            Token::Separator(',') | Token::Paren(')') | Token::Operation('=')
        ) && !declarator(parser, lexer, depth + 1)?
        {
            return Ok(false);
        }
        if lexer.next_if(Token::Operation('='))? {
            while !matches!(lexer.peek()?.0, Token::Separator(',') | Token::Paren(')')) {
                match lexer.peek()?.0 {
                    Token::Paren('(' | '[' | '{') => {
                        if !balanced(lexer, depth + 1)? {
                            return Ok(false);
                        }
                    }
                    Token::End => return Ok(false),
                    _ => {
                        lexer.next()?;
                    }
                }
            }
        }
        if lexer.next_if(Token::Paren(')'))? {
            return Ok(true);
        }
        if !lexer.next_if(Token::Separator(','))? {
            return Ok(false);
        }
    }
}

fn balanced(lexer: &mut Lexer<'_>, depth: u32) -> Result<bool, ParseError> {
    check_depth(lexer, depth)?;
    let closing = match lexer.next()?.0 {
        Token::Paren('(') => ')',
        Token::Paren('[') => ']',
        Token::Paren('{') => '}',
        _ => unreachable!(),
    };
    loop {
        match lexer.peek()?.0 {
            Token::Paren('(' | '[' | '{') => {
                if !balanced(lexer, depth + 1)? {
                    return Ok(false);
                }
            }
            Token::Paren(')' | ']' | '}') => {
                return lexer.next_if(Token::Paren(closing));
            }
            Token::End => return Ok(false),
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
            span: lexer.peek()?.1,
        })
    } else {
        Ok(())
    }
}
