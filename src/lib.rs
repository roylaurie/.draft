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

pub const ACCT_SYSTEM_ID:   SystemID = 1;

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

        assert_eq!(10.00, index.account_for("Office Supplies")?.balance_noncurrent());
        assert_eq!(90.00, index.account(StandardAccounts::Cash)?.balance_noncurrent());
        assert_eq!(100.00, index.account(StandardAccounts::CommonStock)?.balance_noncurrent());


        Ok(())
    }
}
