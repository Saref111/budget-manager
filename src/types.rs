pub struct Budget {
    transactions: Vec<Transaction>,
    total: i32,
    name: String
}

pub struct Transaction {
    id: String,
    sum: i32,
    message: String,
}

impl Budget {
    pub fn new(name: Option<String>, transaction: Option<Vec<Transaction>>, total: i32) -> Self {
        Budget {
            name: name.unwrap_or_default(),
            total,
            transactions: transaction.unwrap_or_default()
        }
    }

    pub fn get_sum(&self) -> i32 {
        self.transactions.iter().map(|t| t.sum).sum()
    }
}

impl Default for Budget {
    fn default() -> Self {
        Budget {
            name: "Default budget".to_string(),
            transactions: vec![],
            total: 0,
        }
    }
}