use crate::{id::*, index::*, definition::{*, standard::*}};

#[derive(Debug)]
pub struct Account {
    definition_id: ID,
    balance: f64
}

impl Account {
    pub fn new(definition: &AccountDefinition) -> Self {
        Self {
            definition_id: definition.id(),
            balance: 0.00
        }
    }

    pub fn standard(definition: StandardAccounts) -> Self {
        Self {
            definition_id: definition.id(),
            balance: 0.00
        }
    }

    pub fn definition<'index>(&self, index: &'index Index) -> &'index AccountDefinition {
        index.definition(self.definition_id).unwrap()
    }

    pub fn definition_id(&self) -> ID {
        self.definition_id
    }

    pub fn set_definition_id(&mut self, definition_id: ID) {
        self.definition_id = definition_id
    }

    pub fn balance(&self) -> f64 {
        self.balance
    }

    pub fn set_balance(&mut self, amount: f64) -> f64 {
        self.balance = amount;
        self.balance
    }

    pub fn increase_balance(&mut self, amount: f64) -> f64 {
        self.balance += amount;
        self.balance

    }

    pub fn decrease_balance(&mut self, amount: f64) -> f64 {
        self.balance -= amount;
        self.balance
    }
}

