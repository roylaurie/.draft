use serde;

pub type Result<T> = ::core::result::Result<T, ()>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ValueChange {
    Debit,
    Credit
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum EquationSide {
    DebitSide,
    CreditSide
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum EquationVariable {
    Assets,
    Liabilities,
    Equity,
}

impl EquationVariable {
    pub fn equation_side(&self) -> EquationSide {
        match self {
            Self::Assets => EquationSide::DebitSide,
            Self::Equity => EquationSide::CreditSide,
            Self::Liabilities => EquationSide::CreditSide,
        }
    }

}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(bound(deserialize = "'de: 'static"))]
pub enum StandardAccountDefinition {
    Expenses(AccountDefinition),
    AccountsReceivable(AccountDefinition),
}

impl StandardAccountDefinition {
    pub fn definition(&self) -> &AccountDefinition {
        match self {
            Self::Expenses(def) => def,
            Self::AccountsReceivable(def) => def,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(bound(deserialize = "'de: 'static"))]
pub enum AccountDefinition {
    Standard(StandardAccountDef)
}

impl AccountDefinitionTrait for AccountDefinition {
    fn name(&self) -> &str {
        match self {
            AccountDefinition::Standard(def) => def.name(),
        }
    }

    fn equation_variable(&self) -> EquationVariable {
        match self {
            AccountDefinition::Standard(def) => def.equation_variable(),
        }
    }

    fn parent(&self) -> Option<&Box<AccountDefinition>> {
        match self {
            AccountDefinition::Standard(def) => def.parent(),
        }
    }
}

impl AccountDefinitionTrait for StandardAccountDefinition {
    fn name(&self) -> &str {
        match self {
            Self::Expenses(def) => def.name(),
            Self::AccountsReceivable(def) => def.name(),
        }
    }

    fn equation_variable(&self) -> EquationVariable {
        match self {
            Self::Expenses(def) => def.equation_variable(),
            Self::AccountsReceivable(def) => def.equation_variable(),
        }
    }

    fn parent(&self) -> Option<&Box<AccountDefinition>> {
        match self {
            Self::Expenses(def) => def.parent(),
            Self::AccountsReceivable(def) => def.parent(),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct StandardAccountDef {
    name: &'static str,
    equation_variable: EquationVariable,
    parent: Option<Box<AccountDefinition>>,
    is_standard: bool
}

pub trait AccountDefinitionTrait {
    fn name(&self) -> &str;
    fn equation_variable(&self) -> EquationVariable;
    fn parent(&self) -> Option<&Box<AccountDefinition>>;
}

impl AccountDefinitionTrait for StandardAccountDef {
    fn name(&self) -> &str {
        &self.name
    }

    fn equation_variable(&self) -> EquationVariable {
        self.equation_variable
    }

    fn parent(&self) -> Option<&Box<AccountDefinition>> {
        self.parent.as_ref()
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(bound(deserialize = "'de: 'static"))]
pub struct Account {
    definition: AccountDefinition,
    amount: f64
}

impl Account {
    pub fn standard(definition: AccountDefinition) -> Self {
        Self {
            definition,
            amount: 0.00
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Currency {
    ticker: &'static str 
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum CommonCurrency {
    USD
}

impl CommonCurrency {
    pub const CURRENCY_USD: Currency = Currency{ ticker: "USD" };

    pub fn currency(&self) -> Currency {
        match self {
            Self::USD => Self::CURRENCY_USD
        }
    }
}

pub struct ChartOfAccounts {
    asset_accounts: Vec<Account>,
    liability_accounts: Vec<Account>,
    equity_accounts: Vec<Account>,
    currency: Currency,
    has_standard_accounts: bool
}

impl ChartOfAccounts {
    pub fn standard(currency: CommonCurrency) -> Self {
        Self {
            asset_accounts: vec![
            ],
            liability_accounts: vec![

            ],
            equity_accounts: vec![

            ],
            currency: currency.currency(),
            has_standard_accounts: true
        }
    }

    pub fn account(&self, definition: StandardAccountDefinition) -> &Account {
        let mut iter = match definition.equation_variable() {
            EquationVariable::Assets => self.asset_accounts.iter(),
            EquationVariable::Liabilities => self.liability_accounts.iter(),
            EquationVariable::Equity => self.equity_accounts.iter()
        };

        iter
            .find(|account| { definition.name() == account.definition.name()})
            .unwrap()
    }

    fn account_mut(&mut self, definition: &StandardAccountDefinition) -> &mut Account {
        let mut iter = match definition.equation_variable() {
            EquationVariable::Assets => self.asset_accounts.iter_mut(),
            EquationVariable::Liabilities => self.liability_accounts.iter_mut(),
            EquationVariable::Equity => self.equity_accounts.iter_mut()
        };

        iter
            .find(|account| { definition.name() == account.definition.name()})
            .unwrap()
    }


    pub fn debit(&mut self, definition: &StandardAccountDefinition, amount: f64) -> Result<f64> {
        let account = self.account_mut(definition);

        match definition.equation_variable().equation_side() {
            EquationSide::DebitSide => account.amount += amount,
            EquationSide::CreditSide => account.amount -= amount,
        }

        Ok(account.amount)
    }

    pub fn credit(&mut self, definition: &StandardAccountDefinition, amount: f64) -> Result<f64> {
        let account = self.account_mut(definition);

        match definition.equation_variable().equation_side() {
            EquationSide::DebitSide => account.amount -= amount,
            EquationSide::CreditSide => account.amount += amount,
        }

        Ok(account.amount)
    }

}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let chart_of_accounts = ChartOfAccounts::standard(CommonCurrency::USD);
    }
}
