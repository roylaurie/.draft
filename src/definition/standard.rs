use strum;
use crate::definition::*;

pub use strum::IntoEnumIterator;
pub use crate::definition::AccountDefinitionTrait; 

#[derive(Debug, strum::EnumIter, PartialEq)]
pub enum StandardAccounts {
    // Assets
    Cash,
    RealEstate,
    Supplies,
    // Liabilities
    AccountsPayable,
    // Equity
    CommonStock,
    /// Money earned in a business. Increases with earnings. Equity.
    Revenue,
    /// Assets consumed in the process of earning revenue. Decreases with costs. Equity.
    Expense,
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
    const REVENUE: &'static AccountDefinition = &StandardAccountDefinition::root(6, "Revenue", Equation::Equity);
    const EXPENSE: &'static AccountDefinition = &StandardAccountDefinition::root(7, "Expense", Equation::Equity);
    // Equity: Revenue
    const FEES_EARNED: &'static AccountDefinition = &StandardAccountDefinition::sub(8, "Expense", Self::REVENUE);

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
            Self::Revenue => &Self::REVENUE,
            Self::Expense => &Self::EXPENSE,
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
