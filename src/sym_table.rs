use std::collections::HashSet;

/// Table of operators.
pub struct OpTable<'a> {
    operators: HashSet<&'a str>,
}

impl<'a> OpTable<'a> {
    /// Creates an [OpTable] from a [HashSet] of operator strings.
    pub fn new(operators: HashSet<&'a str>) -> Self {
        Self { operators }
    }

    /// Checks if the given string is a recognized operator.
    pub fn contains(&self, s: &str) -> bool {
        self.operators.contains(s)
    }
}
