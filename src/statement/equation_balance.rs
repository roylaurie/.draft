use thousands::{self, Separable};

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
        println!("| {:^12} | {:^28} |", debit_side, credit_side);
        println!("+{:=^45}+", '=');
        println!();
    }
}

