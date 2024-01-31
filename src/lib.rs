pub mod error;
pub mod equation;
pub mod currency;
pub mod definition;
pub mod account;
pub mod index;
pub mod statement;
pub mod id;

pub use error::*;
pub use equation::*;
pub use currency::*;
pub use definition::*;
pub use definition::standard::*;
pub use account::*;
pub use index::*;
pub use statement::*;
pub use id::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_account() -> Result<()> {
        let mut index = Index::standard(CommonCurrencies::USD);
        index.create_custom_account(String::from("Office Supplies"), &StandardAccounts::Supplies)?;

        assert_eq!(
            index.find_definition("Office Supplies").unwrap()
                .parent(&index).unwrap().id(),
            StandardAccounts::Supplies.id()
        );

        index.debit(StandardAccounts::Cash, 100.00)?;
        index.credit(StandardAccounts::CommonStock, 100.00)?;

        index.credit(StandardAccounts::Cash, 10.00)?;
        index.debit_for("Office Supplies", 10.00)?;

        assert_eq!(10.00, index.account_for("Office Supplies").balance());
        assert_eq!(90.00, index.account(StandardAccounts::Cash).balance());
        assert_eq!(100.00, index.account(StandardAccounts::CommonStock).balance());


        Ok(())
    }

    #[test]
    fn it_works() -> Result<()> {
        let mut accounts = Index::standard(CommonCurrencies::USD);
        //let mut journal = Journal::new();

        /*journal.entry_builder()
              .debit(StandardAccount::Cash, 25_000.00)
              .credit(StandardAccount::CommonStock, 25_000.00)
              .build();
        
          accounts.process(journal)?;
        })*/
        accounts.debit(StandardAccounts::Cash, 25_000.00)?;
        accounts.credit(StandardAccounts::CommonStock, 25_000.00)?;
        accounts.equation_balance().print_table_entitled("Transaction A");

        accounts.debit(StandardAccounts::RealEstate, 20_000.00)?;
        accounts.credit(StandardAccounts::Cash, 20_000.00)?;
        accounts.equation_balance().print_table_entitled("Transaction B");

        accounts.debit(StandardAccounts::Supplies, 1_350.00)?;
        accounts.credit(StandardAccounts::AccountsPayable, 1_350.00)?;
        accounts.equation_balance().print_table_entitled("Transaction C");

        accounts.debit(StandardAccounts::Cash, 7_500.00)?;
        accounts.credit(StandardAccounts::FeesEarned, 7_500.00)?;
        accounts.equation_balance().print_table_entitled("Transaction D");

        Ok(())
    }
}
