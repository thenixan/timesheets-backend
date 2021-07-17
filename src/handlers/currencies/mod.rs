use rusty_money::crypto::Currency;

pub(crate) static AVAILABLE_CURRENCIES: &'static [&'static str] = &[
    rusty_money::iso::USD.iso_alpha_code,
    rusty_money::iso::EUR.iso_alpha_code,
    rusty_money::iso::RUB.iso_alpha_code,
    rusty_money::iso::UAH.iso_alpha_code,
    rusty_money::crypto::BTC.code,
    rusty_money::crypto::ETH.code,
];
