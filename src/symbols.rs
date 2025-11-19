#[derive(Clone, PartialEq)]
pub enum Symbols {
    Addition,
    Summation,
}

#[derive(Clone)]
pub(crate) struct Definition {
    pub(crate) id: Symbols,
    pub(crate) value: &'static str,
    pub(crate) optional_value: Option<&'static str>,
}

const DEFINITIONS: [Definition; 1] = [Definition {
    id: Symbols::Addition,
    value: "+",
    optional_value: Option::None,
}];

impl Symbols {
    pub fn from_str(string: &'static str) -> Self {
        DEFINITIONS
            .iter()
            .filter_map(
                |d| match (d.value == string, d.optional_value == Option::Some(string)) {
                    (true, _) => Option::Some(d.id.clone()),
                    (_, true) => Option::Some(d.id.clone()),
                    _ => Option::None,
                },
            )
            .into_iter()
            .nth(0)
            .expect(format!("Couldn't Find Matching Symbol").as_str())
    }
}
impl Symbols {
    pub fn as_str(&self) -> &'static str {
        // TODO: DIFFERENT EXPORTS (LIKE TYPST)
        DEFINITIONS
            .iter()
            .filter_map(|d| {
                if self.clone() == d.id {
                    Option::Some(d.value)
                } else {
                    Option::None
                }
            })
            .into_iter()
            .nth(0)
            .expect(format!("Couldn't Find Matching Symbol").as_str())
    }
}
