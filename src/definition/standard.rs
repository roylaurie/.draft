use strum;
use crate::definition::*;

pub use strum::IntoEnumIterator;
pub use crate::definition::AccountDefinitionTrait; 

#[derive(Debug, strum::EnumIter, PartialEq)]
pub enum StandardAccounts {
    // ASSETS 
    Cash,
    RealEstate,
    Supplies,
    // LIABILITIES 
    AccountsPayable,
    // EQUITY 
    CommonStock,
    /// Money earned in a business. Increases with earnings. Equity.
    Revenue,
    /// Assets consumed in the process of earning revenue. Decreases with costs. Equity.
    Dividends,
    Expense,
    // Equity: Expenses
    Wages,
    Rent,
    Utilities,
    SuppliesExpenses,
    MiscExpenses,
    // Equity: Revenue
    /// Revenue generated through the sale of services. Increases with earnings. Decreases with refunds. Equity: Revenue.
    FeesEarned
}

impl StandardAccounts {
    // Assets
    const CASH: &'static AccountDefinition = &StandardAccountDefinition::root(1, "Cash", Equation::Assets);
    const REAL_ESTATE: &'static AccountDefinition = &StandardAccountDefinition::root(2, "Real Estate", Equation::Assets);
    const SUPPLIES: &'static AccountDefinition = &StandardAccountDefinition::root(3, "Supplies", Equation::Assets);
    // Liabilities
    const ACCOUNTS_PAYABLE: &'static AccountDefinition = &StandardAccountDefinition::root(4, "Accounts Payable", Equation::Liabilities);
    // Equity
    const COMMON_STOCK: &'static AccountDefinition = &StandardAccountDefinition::root(5, "Common Stock", Equation::Equity);
    const DIVIDENDS: &'static AccountDefinition = &StandardAccountDefinition::root(6, "Dividends", Equation::Equity);
    const REVENUE: &'static AccountDefinition = &StandardAccountDefinition::root(7, "Revenue", Equation::Equity);
    const EXPENSE: &'static AccountDefinition = &StandardAccountDefinition::root(8, "Expense", Equation::Equity);
    // Equity: Expense
    const WAGES: &'static AccountDefinition = &StandardAccountDefinition::sub(9, "Wages", Self::EXPENSE);
    const RENT: &'static AccountDefinition = &StandardAccountDefinition::sub(10, "Rent", Self::EXPENSE);
    const UTILITIES: &'static AccountDefinition = &StandardAccountDefinition::sub(11, "Utilities", Self::EXPENSE);
    const SUPPLIES_EXPENSES: &'static AccountDefinition = &StandardAccountDefinition::sub(12, "Miscellaneous Expenses", Self::EXPENSE);
    const MISC_EXPENSES: &'static AccountDefinition = &StandardAccountDefinition::sub(13, "Supplies Expenses", Self::EXPENSE);
    // Equity: Revenue
    const FEES_EARNED: &'static AccountDefinition = &StandardAccountDefinition::sub(14, "Fees Earned", Self::REVENUE);

    pub fn definition(&self) -> &'static AccountDefinition {
        match self {
            // Assets
            Self::Cash => &Self::CASH,
            Self::RealEstate => &Self::REAL_ESTATE,
            Self::Supplies => &Self::SUPPLIES,
            // Liabilities
            Self::AccountsPayable => &Self::ACCOUNTS_PAYABLE,
            // Equity
            Self::CommonStock => &Self::COMMON_STOCK,
            Self::Dividends => &Self::DIVIDENDS,
            Self::Revenue => &Self::REVENUE,
            Self::Expense => &Self::EXPENSE,
            Self::Wages => &Self::WAGES,
            Self::Rent => &Self::RENT,
            Self::Utilities => &Self::UTILITIES,
            Self::SuppliesExpenses => &Self::SUPPLIES_EXPENSES,
            Self::MiscExpenses=> &Self::MISC_EXPENSES,
            // Equity: Revenue
            Self::FeesEarned => &Self::FEES_EARNED,
        }
    }
}

impl AsRef<AccountDefinition> for StandardAccounts {
    fn as_ref(&self) -> &AccountDefinition {
        self.definition()
    }
}
