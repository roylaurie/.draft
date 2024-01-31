use std::collections::HashMap;

use crate::{account::*, currency::*, definition::standard::*, equation::*, error::*, id::*, statement::equation_balance::*, definition::*};

/// aka Chart of Accounts
pub struct Index {
    domain: DomainID,
    next_serial: SerialID,
    custom_account_definitions: HashMap<SerialID, AccountDefinition>,
    asset_accounts: Vec<Account>,
    liability_accounts: Vec<Account>,
    equity_accounts: Vec<Account>,
    currency: Currency,
    has_standard_accounts: bool
}

impl Index {
    pub fn currency(&self) -> &Currency {
        &self.currency
    }

    pub fn definition(&self, definition_id: ID) -> Option<&AccountDefinition> {
        match ClassIdentity::from(definition_id.class) {
            ClassIdentity::CustomAccountDefinition => self.custom_account_definitions.get(&definition_id.serial),
            ClassIdentity::StandardAccountDefinition => {
                StandardAccounts::iter()
                    .find(|def| { def.id() == definition_id })
                    .and_then(|std_def| Some(std_def.definition()))
            },
            _ => unreachable!("Unable to fetch AccountDefinition for unsupported Class ID {}", definition_id.class)
        }
    }

    pub fn find_definition(&self, name: &str) -> Option<&AccountDefinition> {
        self.custom_account_definitions.values().find(|def| def.name() == name)
    }

    pub fn find_definition_mut(&mut self, name: &str) -> Option<&AccountDefinition> {
        self.custom_account_definitions.values().find(|def| def.name() == name)
    }

    pub fn has_standard_accounts(&self) -> bool {
        self.has_standard_accounts
    }

    pub fn standard(currency: CommonCurrencies) -> Self {
        Self {
            domain: 0,
            next_serial: 1,
            custom_account_definitions: HashMap::new(),
            asset_accounts: StandardAccounts::iter()
                .filter(|std| std.definition().equation_variable() == Equation::Assets)
                .map(|std| Account::standard(std))
                .collect(),
            liability_accounts: StandardAccounts::iter()
                .filter(|std| std.definition().equation_variable() == Equation::Liabilities)
                .map(|std| Account::standard(std))
                .collect(),
            equity_accounts: StandardAccounts::iter()
                .filter(|std| std.definition().equation_variable() == Equation::Equity)
                .map(|std| Account::standard(std))
                .collect(),
            currency: currency.currency(),
            has_standard_accounts: true
        }
    }

    pub fn account<D: AsRef<AccountDefinition>>(&self, definition: D) -> &Account {
        let definition = definition.as_ref();
        let mut iter = match definition.equation_variable() {
            Equation::Assets => self.asset_accounts.iter(),
            Equation::Liabilities => self.liability_accounts.iter(),
            Equation::Equity => self.equity_accounts.iter()
        };

        let def_id = definition.id();
        iter
            .find(|account| { def_id == account.definition_id() })
            .unwrap()
    }

    pub fn account_for<S: AsRef<str>>(&self, definition_name: S) -> &Account {
        let (def_id, def_eqvar) = {
            let definition = self.find_definition(definition_name.as_ref()).unwrap();
            (definition.id(), definition.equation_variable())
        };

        let mut iter = match def_eqvar {
            Equation::Assets => self.asset_accounts.iter(),
            Equation::Liabilities => self.liability_accounts.iter(),
            Equation::Equity => self.equity_accounts.iter()
        };

        iter
            .find(|account| { account.definition_id() == def_id })
            .unwrap()
    }


    fn account_mut(&mut self, definition_id: ID, equation_variable: Equation) -> &mut Account {
        let mut iter = match equation_variable {
            Equation::Assets => self.asset_accounts.iter_mut(),
            Equation::Liabilities => self.liability_accounts.iter_mut(),
            Equation::Equity => self.equity_accounts.iter_mut()
        };

        iter.find(|account| account.definition_id() == definition_id).unwrap()
    }

    fn next_serial(&mut self) -> SerialID {
        let serial = self.next_serial;
        self.next_serial += 1;
        serial
    }

    pub fn create_custom_account(&mut self, name: String, parent_definition: &impl AccountDefinitionTrait) -> Result<()> {
        let def_id = ID::new(self.domain, CustomAccountDefinition::CLASS_IDENTITY.into(), self.next_serial());
        let definition = AccountDefinition::Custom(CustomAccountDefinition::new(
            def_id, 
            name,
            parent_definition.equation_variable(),
            parent_definition.id()));
        self.custom_account_definitions.insert(def_id.serial, definition);
        let definition = self.custom_account_definitions.get(&def_id.serial).unwrap();

        match definition.equation_variable() {
            Equation::Assets => self.asset_accounts.push(Account::new(&definition)),
            Equation::Liabilities => self.liability_accounts.push(Account::new(&definition)),
            Equation::Equity => self.equity_accounts.push(Account::new(&definition)),
        }

        Ok(())
    }


    pub fn debit<D: AsRef<AccountDefinition>>(&mut self, definition: D, amount: f64) -> Result<f64> {
        let account = self.account_mut(definition.as_ref().id(), definition.as_ref().equation_variable());

        let balance = match definition.as_ref().equation_variable().equation_side() {
            EquationSide::DebitSide => account.increase_balance(amount),
            EquationSide::CreditSide => account.decrease_balance(amount),
        };

        Ok(balance)
    }

    pub fn debit_for<S: AsRef<str>>(&mut self, definition_name: S, amount: f64) -> Result<f64> {
        let (def_id, def_eqvar) = {
            let definition = self.find_definition(definition_name.as_ref()).unwrap();
            (definition.id(), definition.equation_variable())
        };

        let account = self.account_mut(def_id, def_eqvar);

        let balance = match def_eqvar.equation_side() {
            EquationSide::DebitSide => account.increase_balance(amount),
            EquationSide::CreditSide => account.decrease_balance(amount),
        };

        Ok(balance)
    }

    pub fn credit<D: AsRef<AccountDefinition>>(&mut self, definition: D, amount: f64) -> Result<f64> {
        let account = self.account_mut(definition.as_ref().id(), definition.as_ref().equation_variable());

        let balance = match definition.as_ref().equation_variable().equation_side() {
            EquationSide::DebitSide => account.decrease_balance(amount),
            EquationSide::CreditSide => account.increase_balance(amount),
        };

        Ok(balance)
    }

    pub fn credit_for<S: AsRef<str>>(&mut self, definition_name: S, amount: f64) -> Result<f64> {
        let (def_id, def_eqvar) = {
            let definition = self.find_definition(definition_name.as_ref()).unwrap();
            (definition.id(), definition.equation_variable())
        };

        let account = self.account_mut(def_id, def_eqvar);

        let balance = match def_eqvar.equation_side() {
            EquationSide::DebitSide => account.decrease_balance(amount),
            EquationSide::CreditSide => account.increase_balance(amount),
        };

        Ok(balance)
    }


    pub fn equation_balance(&self) -> EquationBalance {
        let assets = self.asset_accounts.iter().map(|a| a.balance()).sum();
        let liabilities = self.liability_accounts.iter().map(|a| a.balance()).sum();
        let equity = self.equity_accounts.iter().map(|a| a.balance()).sum();
        EquationBalance::new(assets, liabilities, equity)
    }

}