pub mod standard;

use std::fmt::Display;
use thousands::{self, Separable};
use strum::IntoEnumIterator;
use crate::standard::*;

pub type Result<T> = ::core::result::Result<T, ()>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueChange {
    Debit,
    Credit
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EquationSide {
    DebitSide,
    CreditSide
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Equation {
    Assets,
    Liabilities,
    Equity,
}

impl Equation {
    pub fn equation_side(&self) -> EquationSide {
        match self {
            Self::Assets => EquationSide::DebitSide,
            Self::Equity => EquationSide::CreditSide,
            Self::Liabilities => EquationSide::CreditSide,
        }
    }

}


#[derive(Debug)]
pub enum AccountDefinition {
    Standard(StandardAccountDef),
    Custom
}

impl AccountDefinitionTrait for AccountDefinition {
    fn name(&self) -> &str {
        match self {
            AccountDefinition::Standard(def) => def.name(),
            AccountDefinition::Custom => todo!(),
        }
    }

    fn equation_variable(&self) -> Equation {
        match self {
            AccountDefinition::Standard(def) => def.equation_variable(),
            AccountDefinition::Custom => todo!(),
        }
    }

    fn parent(&self) -> Option<&AccountDefinition> {
        match self {
            AccountDefinition::Standard(def) => def.parent(),
            AccountDefinition::Custom => todo!(),
        }
    }
}

impl AccountDefinitionTrait for StandardAccount {
    fn name(&self) -> &str {
        self.definition().name()
    }

    fn equation_variable(&self) -> Equation {
        self.definition().equation_variable() 
    }

    fn parent(&self) -> Option<&AccountDefinition> {
        self.definition().parent()
    }
}

#[derive(Debug)]
pub struct StandardAccountDef {
    name: &'static str,
    equation_variable: Equation,
    parent: Option<&'static AccountDefinition>,
}

impl StandardAccountDef {
    pub const fn root(name: &'static str, equation_variable: Equation) -> AccountDefinition {
        AccountDefinition::Standard(Self {
            name,
            equation_variable,
            parent: None,
        })
    }

    pub const fn sub(name: &'static str, parent: &'static AccountDefinition) -> AccountDefinition {
        let parent_def = if let AccountDefinition::Standard(standard_def) = parent {
            standard_def
        } else {
            panic!("Only AccountDefinition::Standard is allowed as parent");
        };

        AccountDefinition::Standard(Self {
            name,
            equation_variable: parent_def.equation_variable_const(),
            parent: Some(parent),
        })
    }

}

pub trait AccountDefinitionTrait {
    fn name(&self) -> &str;
    fn equation_variable(&self) -> Equation;
    fn parent(&self) -> Option<&AccountDefinition>;
}

impl AccountDefinitionTrait for StandardAccountDef {
    fn name(&self) -> &str {
        &self.name
    }

    fn equation_variable(&self) -> Equation {
        self.equation_variable
    }

    fn parent(&self) -> Option<&AccountDefinition> {
        self.parent
    }
}

impl StandardAccountDef {
    pub const fn equation_variable_const(&self) -> Equation {
        self.equation_variable
    }
}

#[derive(Debug)]
pub struct Account {
    custom_definition: Option<AccountDefinition>,
    standard_definition: Option<StandardAccount>,
    amount: f64
}

impl Account {
    pub fn standard(definition: StandardAccount) -> Self {
        Self {
            standard_definition: Some(definition),
            custom_definition: None,
            amount: 0.00
        }
    }
}

impl Account {
    pub fn definition(&self) -> &AccountDefinition {
        if let Some(ref def) = self.standard_definition {
            def.definition()
        } else {
            self.custom_definition.as_ref().unwrap()
        }
    }
}

#[derive(Debug)]
pub struct Currency {
    ticker: &'static str 
}

impl Currency {
    pub fn ticker(&self) -> &str {
        &self.ticker
    }
}

#[derive(Debug)]
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
    pub fn currency(&self) -> &Currency {
        &self.currency
    }

    pub fn has_standard_accounts(&self) -> bool {
        self.has_standard_accounts
    }

    pub fn standard(currency: CommonCurrency) -> Self {
        Self {
            asset_accounts: StandardAccount::iter()
                .filter(|std| std.definition().equation_variable() == Equation::Assets)
                .map(|std| Account::standard(std))
                .collect(),
            liability_accounts: StandardAccount::iter()
                .filter(|std| std.definition().equation_variable() == Equation::Liabilities)
                .map(|std| Account::standard(std))
                .collect(),
            equity_accounts: StandardAccount::iter()
                .filter(|std| std.definition().equation_variable() == Equation::Equity)
                .map(|std| Account::standard(std))
                .collect(),
            currency: currency.currency(),
            has_standard_accounts: true
        }
    }

    pub fn account(&self, definition: StandardAccount) -> &Account {
        let mut iter = match definition.equation_variable() {
            Equation::Assets => self.asset_accounts.iter(),
            Equation::Liabilities => self.liability_accounts.iter(),
            Equation::Equity => self.equity_accounts.iter()
        };

        iter
            .find(|account| { definition.name() == account.definition().name()})
            .unwrap()
    }

    fn account_mut(&mut self, definition: &StandardAccount) -> &mut Account {
        let mut iter = match definition.equation_variable() {
            Equation::Assets => self.asset_accounts.iter_mut(),
            Equation::Liabilities => self.liability_accounts.iter_mut(),
            Equation::Equity => self.equity_accounts.iter_mut()
        };

        iter
            .find(|account| { definition.name() == account.definition().name()})
            .unwrap()
    }


    pub fn debit(&mut self, definition: StandardAccount, amount: f64) -> Result<f64> {
        let account = self.account_mut(&definition);

        match definition.equation_variable().equation_side() {
            EquationSide::DebitSide => account.amount += amount,
            EquationSide::CreditSide => account.amount -= amount,
        }

        Ok(account.amount)
    }

    pub fn credit(&mut self, definition: StandardAccount, amount: f64) -> Result<f64> {
        let account = self.account_mut(&definition);

        match definition.equation_variable().equation_side() {
            EquationSide::DebitSide => account.amount -= amount,
            EquationSide::CreditSide => account.amount += amount,
        }

        Ok(account.amount)
    }

    pub fn equation_balance(&self) -> EquationBalance {
        let assets = self.asset_accounts.iter().map(|a| a.amount).sum();
        let liabilities = self.liability_accounts.iter().map(|a| a.amount).sum();
        let equity = self.equity_accounts.iter().map(|a| a.amount).sum();
        EquationBalance::new(assets, liabilities, equity)
    }

}

#[derive(Debug)]
pub struct EquationBalance {
    assets: f64,
    liabilities: f64,
    equity: f64,
    debit_side: f64,
    credit_side: f64
}

impl EquationBalance {
    pub fn new(assets: f64, liabilities: f64, equity: f64) -> Self {
        Self {
            assets,
            liabilities,
            equity,
            debit_side: assets,
            credit_side: liabilities + equity
        }
    }

    pub fn print_table(&self) {
        self.print_table_ext(None)
    }

    pub fn print_table_entitled(&self, title: &str) {
        self.print_table_ext(Some(title))
    }

    fn print_table_ext(&self, title: Option<&str>) {
        let assets = self.assets.separate_with_commas();
        let liabilities = self.liabilities.separate_with_commas(); 
        let equity = self.equity.separate_with_commas();
        let credit_side = self.credit_side.separate_with_commas();
        let debit_side = self.debit_side.separate_with_commas();
        let title = title
            .and_then(|t| Some(format!("[ {t} ]")))
            .unwrap_or_default();

        println!();
        println!("+{:=^45}+", title);
        println!("| {:^12} = {:^13} + {:^12} |", "assets", "liabilities", "equity");
        println!("|-{:-^12}-|-{:-^13}-|-{:-^12}-|", '-', '-', '-');
        println!("| {:^12} | {:^13} | {:^12} |", assets, liabilities, equity);
        println!("|{:-^45}|", '-');
        println!("| {:^12} | {:^28} |", credit_side, debit_side);
        println!("+{:=^45}+", '=');
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        let mut accounts = ChartOfAccounts::standard(CommonCurrency::USD);
        //let mut journal = Journal::new();

        /*journal.entry_builder()
              .debit(StandardAccount::Cash, 25_000.00)
              .credit(StandardAccount::CommonStock, 25_000.00)
              .build();
        
          accounts.process(journal)?;
        })*/
        accounts.debit(StandardAccount::Cash, 25_000.00)?;
        accounts.credit(StandardAccount::CommonStock, 25_000.00)?;
        accounts.equation_balance().print_table_entitled("Transaction A");

        accounts.debit(StandardAccount::RealEstate, 20_000.00)?;
        accounts.credit(StandardAccount::Cash, 20_000.00)?;
        accounts.equation_balance().print_table_entitled("Transaction B");

        accounts.debit(StandardAccount::Supplies, 1_350.00)?;
        accounts.credit(StandardAccount::AccountsPayable, 1_350.00)?;
        accounts.equation_balance().print_table_entitled("Transaction C");

        accounts.debit(StandardAccount::Cash, 7_500.00)?;
        accounts.credit(StandardAccount::FeesEarned, 7_500.00)?;
        accounts.equation_balance().print_table_entitled("Transaction D");

        Ok(())
    }
}
