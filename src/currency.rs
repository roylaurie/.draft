pub trait CurrencyTrait {
    fn ticker(&self) -> &str;
    fn symbol(&self) -> &str;
    fn name(&self) -> &str;
}

pub enum Currency {
    Common(&'static CommonCurrency),
    Custom(CustomCurrency)
}

impl CurrencyTrait for Currency {
    fn ticker(&self) -> &str {
        match self {
            Self::Common(currency) => currency.ticker(),
            Self::Custom(currency) => currency.ticker(),
        }
    }

    fn symbol(&self) -> &str {
        match self {
            Self::Common(currency) => currency.symbol(),
            Self::Custom(currency) => currency.symbol(),
        }
    }

    fn name(&self) -> &str {
        match self {
            Self::Common(currency) => currency.name(),
            Self::Custom(currency) => currency.name(),
        }
    }
}

pub enum CommonCurrencies {
    USD
}

impl CommonCurrencies {
    pub const CURRENCY_USD: Currency = Currency::Common(&CommonCurrency::new("USD", "$", "US Dollar"));

    pub fn currency(&self) -> Currency {
        match self {
            Self::USD => Self::CURRENCY_USD
        }
    }
}

#[derive(Debug)]
pub struct CommonCurrency {
    ticker: &'static str,
    symbol: &'static str,
    name: &'static str,
}

impl CurrencyTrait for CommonCurrency {
    fn ticker(&self) -> &str {
        self.ticker
    }

    fn symbol(&self) -> &str {
        self.symbol
    }

    fn name(&self) -> &str {
        self.name
    }
}

impl CommonCurrency {
    pub const fn new(ticker: &'static str, symbol: &'static str, name: &'static str) -> Self {
        Self {
            ticker,
            symbol,
            name
        }
    }

    pub fn currency(&'static self) -> Currency {
        Currency::Common(self)
    }
}


#[derive(Debug)]
pub struct CustomCurrency {
    ticker: String,
    symbol: String,
    name: String,
}

impl CurrencyTrait for CustomCurrency {
    fn ticker(&self) -> &str {
        &self.ticker
    }

    fn symbol(&self) -> &str {
        &self.symbol
    }

    fn name(&self) -> &str {
        &self.name
    }
}