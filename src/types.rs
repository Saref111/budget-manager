use rusqlite::{Connection, Result as DBResult};
use tui::widgets::ListState;

use crate::db::budget::get_all_budgets;

#[derive(PartialEq)]
pub enum AppMode {
    Normal,
    Input
}

pub struct App {
    pub mode: AppMode,
    pub input: String,
    pub entity: (String, Option<i32>),
    pub budgets: Vec<Budget>,
    pub active_tab: usize,
    pub list_state: ListState,
    pub conn: Connection,
    pub need_update: bool,
}

impl App {
    pub fn new(conn: Connection) -> Self {
        Self { 
            mode: AppMode::Normal, 
            input: String::new(), 
            entity: (String::new(), None), 
            budgets: vec![],
            active_tab: 0, 
            list_state: ListState::default(), 
            conn,
            need_update: true
        }
    }

    pub fn update(&mut self) -> DBResult<()> {
        self.budgets = get_all_budgets(&self.conn)?;

        Ok(())
    }

    pub fn update_if_need(&mut self) -> DBResult<()> {
        if self.need_update {
            self.update()?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum UserActions {
    Exit,
    Continue,
    IntoInputMode, // ???
    IntoNormalMode, // ???

    AddTransaction(PartialBudgetTransaction, u32),
    UpdateTransaction(BudgetTransaction),
    RemoveTransaction(u32),

    AddBudget(MinimalBudget),
    UpdateBudget(PartialBudget),
    RemoveBudget(u32),
}

#[derive(Debug)]
pub struct MinimalBudget {
    pub total: i32,
    pub name: String,
}

#[derive(Debug)]
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

pub trait SavableBudget {
    fn prepare_for_db(&self) -> MinimalBudget;
    fn get_without_transactions(&self) -> PartialBudget;
}

impl SavableBudget for PartialBudget {
    fn prepare_for_db(&self) -> MinimalBudget {
        MinimalBudget {
            total: self.total,
            name: self.name.to_owned(),
        }
    }

    fn get_without_transactions(&self) -> PartialBudget {
        PartialBudget {
            id: self.id,
            total: self.total,
            name: self.name.to_owned(),
        }
    }
}

impl SavableBudget for Budget {
    fn prepare_for_db(&self) -> MinimalBudget {
        MinimalBudget {
            total: self.total,
            name: self.name.to_owned(),
        }
    }

    fn get_without_transactions(&self) -> PartialBudget {
        PartialBudget {
            id: self.id,
            total: self.total,
            name: self.name.to_owned(),
        }
    }
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

#[derive(Debug)]
pub struct BudgetTransaction {
    pub id: String,
    pub sum: i32,
    pub message: String,
}

#[derive(Debug)]
pub struct PartialBudgetTransaction {
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