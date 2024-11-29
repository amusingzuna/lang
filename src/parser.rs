use std::sync::Arc;

pub struct Parser<'input, T: 'input>(
    Arc<dyn Fn(&'input str) -> Result<(T, &'input str), &'static str> + 'input>,
);

impl<'input, T: 'input> Clone for Parser<'input, T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<'a, T> Parser<'a, T> {
    pub fn new<F>(parser: F) -> Self
    where
        F: 'a + Fn(&'a str) -> Result<(T, &'a str), &'static str>,
    {
        Self(Arc::new(parser))
    }

    pub fn lazy<F>(parser_factory: F) -> Self
    where
        F: Fn() -> Self + 'a,
    {
        Parser::new(move |input: &'a str| {
            let parser = parser_factory();
            parser.parse(input)
        })
    }

    pub fn map<U, F>(self, func: F) -> Parser<'a, U>
    where
        F: 'a + Fn(T) -> U,
        U: 'a,
    {
        Parser::new(move |input: &'a str| {
            self.parse(input)
                .map(|(result, remaining)| (func(result), remaining))
        })
    }

    pub fn pure(a: T) -> Self
    where
        T: 'a + Clone,
    {
        Parser::new(move |input: &'a str| Ok((a.clone(), input)))
    }

    pub fn many(self) -> Parser<'a, Vec<T>> {
        Parser::new(move |mut input: &'a str| {
            let mut results = Vec::new();
            while let Ok((result, remaining)) = self.parse(input) {
                results.push(result);
                input = remaining;
            }
            Ok((results, input))
        })
    }

    pub fn some(self) -> Parser<'a, Vec<T>> {
        self.clone().and(self.many()).map(|(head, rest)| {
            let mut result = Vec::with_capacity(rest.len() + 1);
            result.push(head);
            result.extend(rest);
            result
        })
    }

    pub fn and<U>(self, other: Parser<'a, U>) -> Parser<'a, (T, U)>
    where
        U: 'a,
    {
        Parser::new(move |input: &'a str| {
            self.parse(input).and_then(|(result_a, remaining_a)| {
                other
                    .parse(remaining_a)
                    .map(|(result_b, remaining_b)| ((result_a, result_b), remaining_b))
            })
        })
    }

    pub fn left<U>(self, other: Parser<'a, U>) -> Parser<'a, T>
    where
        U: 'a,
    {
        let merge = self.and(other);

        Parser::new(move |input: &'a str| match merge.parse(input) {
            Ok(((left, _), remaining)) => Ok((left, remaining)),
            Err(reason) => Err(reason),
        })
    }

    pub fn right<U>(self, other: Parser<'a, U>) -> Parser<'a, U>
    where
        U: 'a,
    {
        let merge = self.and(other);

        Parser::new(move |input: &'a str| match merge.parse(input) {
            Ok(((_, right), remaining)) => Ok((right, remaining)),
            Err(reason) => Err(reason),
        })
    }

    pub fn empty(reason: &'static str) -> Self {
        Parser::new(move |_| Err(reason))
    }

    pub fn or(self, other: Parser<'a, T>) -> Parser<'a, T> {
        Parser::new(move |input: &'a str| self.parse(input).or_else(|_| other.parse(input)))
    }

    pub fn parse(&self, input: &'a str) -> Result<(T, &'a str), &'static str> {
        (self.0)(input)
    }
}

impl<'a> Parser<'a, Vec<char>> {
    pub fn qualify(self) -> Parser<'a, String> {
        self.map(|x| x.into_iter().collect())
    }
}

pub fn any() -> Parser<'static, char> {
    Parser::new(move |input: &'static str| {
        let mut chars = input.chars();
        match chars.next() {
            Some(c) => Ok((c, chars.as_str())),
            _ => Err("Nothing to parse"),
        }
    })
}

pub fn char(expected: char) -> Parser<'static, char> {
    Parser::new(move |input: &'static str| {
        let mut chars = input.chars();
        match chars.next() {
            Some(c) if c == expected => Ok((c, chars.as_str())),
            _ => Err("Character mismatch"),
        }
    })
}

pub fn list(allowed: &[char]) -> Parser<'static, char> {
    let fail = Parser::empty("List parser has no members");
    allowed.iter().fold(fail, |sum, x| sum.or(char(*x)))
}

pub fn whitespace() -> Parser<'static, char> {
    list(&[' ', '\n', '\t', '\r'])
}

pub fn lowercase() -> Parser<'static, char> {
    list(&[
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ])
}

pub fn uppercase() -> Parser<'static, char> {
    list(&[
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ])
}

pub fn letter() -> Parser<'static, char> {
    lowercase().or(uppercase())
}

pub fn digit() -> Parser<'static, char> {
    list(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'])
}

pub fn alphanumeric() -> Parser<'static, char> {
    letter().or(digit())
}

pub fn between<T, U, P>(
    a: Parser<'static, T>,
    b: Parser<'static, U>,
    c: Parser<'static, P>,
) -> Parser<'static, U> {
    a.right(b).left(c)
}

pub fn otherwise<T: Clone>(b: Parser<'static, T>, a: T) -> Parser<'static, T> {
    b.or(Parser::pure(a))
}

pub fn delimited<T: Clone, U>(
    a: Parser<'static, T>,
    sep: Parser<'static, U>,
) -> Parser<'static, Vec<T>> {
    otherwise(
        a.clone().and(sep.right(a).many()).map(|(first, mut rest)| {
            rest.insert(0, first);
            rest
        }),
        vec![],
    )
}

pub fn strip<T>(a: Parser<'static, T>) -> Parser<'static, T> {
    between(whitespace().many(), a, whitespace().many())
}

pub fn string(input: &'static str) -> Parser<'static, String> {
    if input.is_empty() {
        return Parser::pure("".to_string());
    }

    char(input.chars().next().unwrap())
        .and(string(&input[1..]))
        .map(|(c, rest)| {
            let mut result = "".to_string();
            result.push(c);
            result.push_str(&rest);
            result
        })
}

pub fn identifier() -> Parser<'static, String> {
    strip(letter().and(alphanumeric().many()).map(|(first, rest)| {
        let mut result = String::new();
        result.push(first);
        result.push_str(&rest.iter().collect::<String>());
        result
    }))
}

pub fn symbol(a: &'static str) -> Parser<'static, String> {
    strip(string(a))
}

pub fn tuple<T>(a: Parser<'static, T>) -> Parser<'static, T> {
    between(symbol("("), a, symbol(")"))
}

pub fn set<T>(a: Parser<'static, T>) -> Parser<'static, T> {
    between(symbol("["), a, symbol("]"))
}

pub fn block<T>(a: Parser<'static, T>) -> Parser<'static, T> {
    between(symbol("{"), a, symbol("}"))
}

pub fn integer() -> Parser<'static, String> {
    strip(digit().some().map(|c| c.into_iter().collect()))
}

pub fn float() -> Parser<'static, String> {
    strip(
        digit()
            .some()
            .qualify()
            .and(symbol("."))
            .and(digit().many().qualify())
            .left(symbol("f").many())
            .map(|((a, b), c)| {
                let mut result = String::new();
                result.push_str(a.as_str());
                result.push_str(b.as_str());
                result.push_str(c.as_str());
                result
            }),
    )
}
