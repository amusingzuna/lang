pub struct Parser<'input, T>(
    Box<dyn Fn(&'input str) -> Result<(T, &'input str), &'static str> + 'input>,
)
where
    T: 'input;

impl<'input, T> Parser<'input, T> {
    pub fn new<F>(parser: F) -> Self
    where
        F: 'input + Fn(&'input str) -> Result<(T, &'input str), &'static str>,
    {
        Self(Box::new(parser))
    }

    pub fn pure(a: T) -> Self
    where
        T: Clone + Copy,
    {
        Parser::new(move |input: &'input str| Ok((a, input)))
    }

    pub fn impure(reason: &'static str) -> Self {
        Parser::new(move |_| Err(reason))
    }

    pub fn or(self, other: Parser<'input, T>) -> Parser<'input, T> {
        Parser::new(move |input: &'input str| self.parse(input).or_else(|_| other.parse(input)))
    }

    pub fn and<U>(self, other: Parser<'input, U>) -> Parser<'input, (T, U)>
    where
        U: 'input,
    {
        Parser::new(move |input: &'input str| {
            self.parse(input).and_then(|(result_a, remaining_a)| {
                other
                    .parse(remaining_a)
                    .map(|(result_b, remaining_b)| ((result_a, result_b), remaining_b))
            })
        })
    }

    pub fn left<U>(self, other: Parser<'input, U>) -> Parser<'input, T>
    where
        U: 'input,
    {
        let merge = self.and(other);

        Parser::new(move |input: &'input str| match merge.parse(input) {
            Ok(((left, _), remaining)) => Ok((left, remaining)),
            Err(reason) => Err(reason),
        })
    }

    pub fn right<U>(self, other: Parser<'input, U>) -> Parser<'input, U>
    where
        U: 'input,
    {
        let merge = self.and(other);

        Parser::new(move |input: &'input str| match merge.parse(input) {
            Ok(((_, right), remaining)) => Ok((right, remaining)),
            Err(reason) => Err(reason),
        })
    }

    pub fn parse(&self, input: &'input str) -> Result<(T, &'input str), &'static str> {
        (self.0)(input)
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
    allowed
        .iter()
        .fold(Parser::impure("List parser has no members"), |sum, x| {
            sum.or(char(*x))
        })
}
