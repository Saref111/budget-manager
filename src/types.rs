pub struct PartialBudget {
    pub total: i32,
    pub name: String,
    pub id: u32
}

pub struct Budget {
    pub transactions: Vec<BudgetTransaction>,
    pub total: i32,
    pub name: String,
    pub id: u32
}

impl Budget {
    pub fn new(name: Option<String>, transaction: Option<Vec<BudgetTransaction>>, total: i32, id: u32) -> Self {
        Budget {
            name: name.unwrap_or ("Primary budget".to_string()),
            total,
            transactions: transaction.unwrap_or_default(),
            id
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
            id: 0
        }
    }
}


pub struct BudgetTransaction {
    pub id: String,
    pub sum: i32,
    pub message: String,
}

impl BudgetTransaction {
    pub fn new(sum: i32, message: String, id: String) -> Self {
        Self {
            sum,
            message,
            id
        }
    }
}