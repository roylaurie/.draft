use crate::*;

#[derive(Debug, strum::EnumIter, PartialEq)]
pub enum StandardAccount {
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

impl StandardAccount {
    // Assets
    const CASH: &'static AccountDefinition = &StandardAccountDef::root("Cash", Equation::Assets);
    const REAL_ESTATE: &'static AccountDefinition = &StandardAccountDef::root("Real Estate", Equation::Assets);
    const SUPPLIES: &'static AccountDefinition = &StandardAccountDef::root("Supplies", Equation::Assets);
    // Liabilities
    const ACCOUNTS_PAYABLE: &'static AccountDefinition = &StandardAccountDef::root("Accounts Payable", Equation::Liabilities);
    // Equity
    const COMMON_STOCK: &'static AccountDefinition = &StandardAccountDef::root("Common Stock", Equation::Equity);
    const REVENUE: &'static AccountDefinition = &StandardAccountDef::root("Revenue", Equation::Equity);
    const EXPENSE: &'static AccountDefinition = &StandardAccountDef::root("Expense", Equation::Equity);
    // Equity: Revenue
    const FEES_EARNED: &'static AccountDefinition = &StandardAccountDef::sub("Expense", Self::REVENUE);

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
