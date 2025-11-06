pub enum Symbols {
    Addition,
    Summation,
}

impl Symbols {
    pub fn as_str(&self) -> &'static str {
        // TODO: DIFFERENT EXPORTS (LIKE TYPST)
        match self {
            Symbols::Addition => "+",
            Symbols::Summation => "∑",
        }
    }
}
