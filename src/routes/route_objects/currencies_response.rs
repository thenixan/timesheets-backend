use serde::Serialize;

#[derive(Serialize)]
pub struct CurrenciesResponse<'r> {
    pub currencies: Vec<CurrencyResponseEntry<'r>>,
}

#[derive(Serialize, Hash, Eq, PartialEq, Debug)]
pub struct CurrencyResponseEntry<'r> {
    pub name: &'r str,
}

impl<'r> From<&'r str> for CurrencyResponseEntry<'r> {
    fn from(s: &'r str) -> Self {
        CurrencyResponseEntry { name: s }
    }
}
