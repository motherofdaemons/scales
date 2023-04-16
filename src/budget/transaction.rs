#[derive(Debug)]
pub struct Transaction {
    payee: String,
    desc: Option<String>,
    amount: i32,
}

impl Transaction {
    pub fn new(payee: String, desc: Option<String>, amount: i32) -> Self {
        Self {
            payee,
            desc,
            amount,
        }
    }

    pub fn get_amount(&self) -> i32 {
        self.amount
    }
}
