pub struct Budget {
    transactions: Vec<Transaction>,
}

pub struct Transaction {
    id: String,
    sum: i32,
    message: String,
}

impl Budget {
    pub fn new(transaction: Option<Vec<Transaction>>) -> Self {
        Budget {
            transactions: transaction.unwrap_or_default()
        }
    }

    pub fn get_total_sum(&self) -> i32 {
        self.transactions.iter().map(|t| t.sum).sum()
    }
}

impl Default for Budget {
    fn default() -> Self {
        Budget {
            transactions: vec![]
        }
    }
}