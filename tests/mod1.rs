
#[cfg(test)]
pub mod tests {
    use acct201::*;

        //let mut journal = Journal::new();

        /*journal.entry_builder()
              .debit(StandardAccount::Cash, 25_000.00)
              .credit(StandardAccount::CommonStock, 25_000.00)
              .build();
        
          accounts.process(journal)?;
        })*/

    #[test]
    fn ch1() -> Result<()> {
        let mut accounts = Index::standard(CommonCurrencies::USD);
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

        accounts.debit(StandardAccounts::Wages, 2_125.00)?;
        accounts.credit(StandardAccounts::Cash, 2_125.00)?;
        accounts.debit(StandardAccounts::Rent, 800.00)?;
        accounts.credit(StandardAccounts::Cash, 800.00)?;
        accounts.debit(StandardAccounts::Utilities, 450.00)?;
        accounts.credit(StandardAccounts::Cash, 450.00)?;
        accounts.debit(StandardAccounts::MiscExpenses, 275.00)?;
        accounts.credit(StandardAccounts::Cash, 275.00)?;
        accounts.equation_balance().print_table_entitled("Transaction E");

        accounts.credit(StandardAccounts::Cash, 950.00)?;
        accounts.debit(StandardAccounts::AccountsPayable, 950.00)?;
        accounts.equation_balance().print_table_entitled("Transaction F");

        let supplies_diff = 550.00 - accounts.account(StandardAccounts::Supplies)?.balance_noncurrent();
        accounts.debit(StandardAccounts::Supplies, supplies_diff)?;
        accounts.credit(StandardAccounts::SuppliesExpenses, supplies_diff)?;
        accounts.equation_balance().print_table_entitled("Transaction G");

        accounts.credit(StandardAccounts::Cash, 2_000.00)?;
        accounts.debit(StandardAccounts::Dividends, 2_000.00)?;
        accounts.equation_balance().print_table_entitled("Transaction H");

        accounts.print_accounts();

        Ok(())
    }
}