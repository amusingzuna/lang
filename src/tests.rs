#[cfg(test)]
mod library {
    use crate::parser::*;

    #[test]
    fn functor_parsers() {
        assert_eq!(
            alphanumeric().map(|c| c.is_alphabetic()).parse("a"),
            Ok((true, ""))
        );
        assert_eq!(
            alphanumeric().map(|c| c.is_alphabetic()).parse("2"),
            Ok((false, ""))
        );
    }

    #[test]
    fn applicative_parsers() {
        assert_eq!(Parser::pure('a').parse("bc"), Ok(('a', "bc")));
        assert_eq!(Parser::pure('b').parse(""), Ok(('b', "")));

        let a_and_b_parser = char('a').and(char('b'));

        assert_eq!(a_and_b_parser.parse("abc"), Ok((('a', 'b'), "c")));
        assert_eq!(a_and_b_parser.parse("aac"), Err("Character mismatch"));

        let a_left_b_parser = char('a').left(char('b'));

        assert_eq!(a_left_b_parser.parse("abc"), Ok(('a', "c")));

        let a_right_b_parser = char('a').right(char('b'));

        assert_eq!(a_right_b_parser.parse("abc"), Ok(('b', "c")));
    }

    #[test]
    fn alternative_parsers() {
        assert_eq!(Parser::<()>::empty("failed").parse("abc"), Err("failed"));

        let a_or_b_parser = char('a').or(char('b'));

        assert_eq!(a_or_b_parser.parse("abc"), Ok(('a', "bc")));
        assert_eq!(a_or_b_parser.parse("bbc"), Ok(('b', "bc")));
        assert_eq!(a_or_b_parser.parse("cbc"), Err("Character mismatch"));
    }

    #[test]
    fn parse_any() {
        assert_eq!(any().parse("abc"), Ok(('a', "bc")));
        assert_eq!(any().parse(""), Err("Nothing to parse"));
    }

    #[test]
    fn parse_char() {
        assert_eq!(char('a').parse("abc"), Ok(('a', "bc")));
        assert_eq!(char('a').parse("xyz"), Err("Character mismatch"));
        assert_eq!(char('a').parse(""), Err("Character mismatch"));
    }

    #[test]
    fn parse_many() {
        assert_eq!(
            alphanumeric().many().parse("abc "),
            Ok((vec!['a', 'b', 'c'], " "))
        );

        assert_eq!(
            whitespace().many().parse("    a"),
            Ok((vec![' ', ' ', ' ', ' '], "a"))
        );
    }

    #[test]
    fn parse_list() {
        let a_or_b_parser = list(&['a', 'b']);

        assert_eq!(a_or_b_parser.parse("abc"), Ok(('a', "bc")));
        assert_eq!(a_or_b_parser.parse("bbc"), Ok(('b', "bc")));
        assert_eq!(a_or_b_parser.parse("cbc"), Err("Character mismatch"));

        let empty_list = list(&[]);

        assert_eq!(empty_list.parse("abc"), Err("List parser has no members"));
    }

    #[test]
    fn parse_between() {
        let between_parser = between(digit(), letter(), digit());

        assert_eq!(between_parser.parse("1a1"), Ok(('a', "")));
        assert_eq!(between_parser.parse("aa1"), Err("Character mismatch"));
    }

    #[test]
    fn parse_otherwise() {
        let option_parser = otherwise(char('b'), 'a');

        assert_eq!(option_parser.parse("ba"), Ok(('b', "a")));
        assert_eq!(option_parser.parse("ab"), Ok(('a', "ab")));
    }

    #[test]
    fn parse_delimited() {
        let delimited_parser = delimited(alphanumeric(), char(','));

        assert_eq!(
            delimited_parser.parse("1,2,3,4"),
            Ok((vec!['1', '2', '3', '4'], ""))
        );

        assert_eq!(
            delimited_parser.parse("1,2,3,,5"),
            Ok((vec!['1', '2', '3'], ",,5"))
        );
    }

    #[test]
    fn parse_strip() {
        let strip_parser = strip(alphanumeric());

        assert_eq!(strip_parser.parse("  a  "), Ok(('a', "")));
    }

    #[test]
    fn parse_string() {
        let string_parser = string("hello");

        assert_eq!(string_parser.parse("hello"), Ok(("hello".to_string(), "")));
        assert_eq!(string_parser.parse("yello"), Err("Character mismatch"));
    }

    #[test]
    fn parse_identifier() {
        assert_eq!(identifier().parse("1id"), Err("Character mismatch"));
        assert_eq!(
            identifier().parse("helloIAmIdent"),
            Ok(("helloIAmIdent".to_string(), ""))
        );
    }

    #[test]
    fn parse_symbol() {
        assert_eq!(symbol("if").parse("if"), Ok(("if".to_string(), "")));
        assert_eq!(symbol("if").parse("else"), Err("Character mismatch"));
    }

    #[test]
    fn parse_accumulators() {
        assert_eq!(
            tuple(identifier()).parse("(abc)"),
            Ok(("abc".to_string(), ""))
        );
        assert_eq!(
            set(identifier()).parse("[abc]"),
            Ok(("abc".to_string(), ""))
        );
        assert_eq!(
            block(identifier()).parse("{abc}"),
            Ok(("abc".to_string(), ""))
        );
    }

    #[test]
    fn parse_integer() {
        assert_eq!(integer().parse("483943"), Ok(("483943".to_string(), "")));
        assert_eq!(
            integer().parse("483943jkds"),
            Ok(("483943".to_string(), "jkds"))
        );
    }

    #[test]
    fn parse_float() {
        assert_eq!(float().parse("12.32f"), Ok(("12.32".to_string(), "")));
        assert_eq!(float().parse("12.32"), Ok(("12.32".to_string(), "")));
        assert_eq!(float().parse("12."), Ok(("12.".to_string(), "")));
        assert_eq!(float().parse("12"), Err("Character mismatch"));
    }
}

#[cfg(test)]
mod syntax {
    use crate::{
        ast::*,
        expression::{block_expr, literal_expr},
        literal::*,
        program,
        statement::*,
        types::*,
    };

    #[test]
    fn parse_primitive() {
        assert_eq!(
            atomic().parse("i32"),
            Ok((Type::Atomic("i32".to_string()), ""))
        );
    }

    #[test]
    fn parse_float_literal() {
        assert_eq!(
            float_literal().parse("12378.5"),
            Ok((Literal::Float("12378.5".to_string()), ""))
        )
    }

    #[test]
    fn parse_integer_literal() {
        assert_eq!(
            integer_literal().parse("12378"),
            Ok((Literal::Integer("12378".to_string()), ""))
        )
    }

    #[test]
    fn parse_boolean_literal() {
        assert_eq!(
            bool_literal().parse("true"),
            Ok((Literal::Boolean(true), ""))
        )
    }

    #[test]
    fn parse_reference_literal() {
        assert_eq!(
            reference_literal().parse("jlfd3"),
            Ok((Literal::Reference("jlfd3".to_string()), ""))
        )
    }

    #[test]
    fn parse_literal_expr() {
        assert_eq!(
            literal_expr().parse("123"),
            Ok((Expression::Literal(Literal::Integer("123".to_string())), ""))
        )
    }

    #[test]
    fn parse_block_expr() {
        assert_eq!(
            block_expr().parse("{let a = 123;}"),
            Ok((
                Expression::Block(vec![Statement::Instantiate(
                    None,
                    "a".to_string(),
                    Expression::Literal(Literal::Integer("123".to_string()))
                )]),
                ""
            ))
        )
    }

    #[test]
    fn parse_declare() {
        assert_eq!(
            declare().parse("let a: int"),
            Ok((
                Statement::Declare(Some(Type::Atomic("int".to_string())), "a".to_string()),
                ""
            ))
        );
        assert_eq!(
            declare().parse("let a"),
            Ok((Statement::Declare(None, "a".to_string()), ""))
        );
    }

    #[test]
    fn parse_assignment() {
        assert_eq!(
            assign().parse("a = 50"),
            Ok((
                Statement::Assign(
                    "a".to_string(),
                    Expression::Literal(Literal::Integer("50".to_string()))
                ),
                ""
            ))
        )
    }

    #[test]
    fn parse_instantiate() {
        assert_eq!(
            instantiate().parse("let a: i32 = 50"),
            Ok((
                Statement::Instantiate(
                    Some(Type::Atomic("i32".to_string())),
                    "a".to_string(),
                    Expression::Literal(Literal::Integer("50".to_string()))
                ),
                ""
            ))
        );
        assert_eq!(
            instantiate().parse("let a = 50"),
            Ok((
                Statement::Instantiate(
                    None,
                    "a".to_string(),
                    Expression::Literal(Literal::Integer("50".to_string()))
                ),
                ""
            ))
        );
    }

    #[test]
    fn parse_program() {
        assert_eq!(
            program().parse("let a;"),
            Ok((Program(vec![Statement::Declare(None, "a".to_string())]), ""))
        );
    }
}
