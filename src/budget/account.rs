use std::fmt::Display;

use leptos::{tracing::info, view, IntoView};

use super::transaction::Transaction;

pub struct Account {
    name: String,
    transactions: Vec<Transaction>,
}

impl Account {
    pub fn new(name: String) -> Self {
        Self {
            name,
            transactions: vec![],
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        info!("Adding transaction {:?}", transaction);
        self.transactions.push(transaction);
    }

    pub fn calculate_balance(&self) -> i32 {
        self.transactions.iter().map(|t| t.get_amount()).sum()
    }
}

impl Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let account_name = &self.name;
        let balance = self.calculate_balance();
        let dollars = balance / 100;
        let cents = balance % 100;
        write!(f, "{account_name}: Balance {dollars}.{cents}")
    }
}
