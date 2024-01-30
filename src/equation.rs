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