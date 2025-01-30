use rusqlite::{Connection, Result as DBResult};
use tui::widgets::ListState;

use crate::db::{
    budget::{
        add_budget, 
        get_all_budgets, 
        remove_budget, 
        update_budget
    }, 
    transaction::{
        add_transaction, 
        remove_transaction, 
        update_transaction
    }
};

#[derive(PartialEq)]
pub enum AppMode {
    Normal,
    InputNewBudget,
    UpdateBudget(u32),

    InputNewTransaction(u32),
    UpdateTransaction(u32),
}

pub struct App {
    pub mode: AppMode,
    pub input: String,
    pub entity: (String, Option<i32>),
    pub budgets: Vec<Budget>,
    pub active_tab: usize,
    pub list_state: ListState,
    conn: Connection,
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
        }
    }

    pub fn update(&mut self) -> DBResult<()> {
        self.budgets = get_all_budgets(&self.conn)?;

        Ok(())
    }

    pub fn add_new_budget(&mut self, b: MinimalBudget) -> DBResult<()> {
        add_budget(&self.conn, Box::new(b))?;
        self.update()
    }

    pub fn remove_budget(&mut self, id: u32) -> DBResult<()> {
        remove_budget(&self.conn, id)?;
        self.active_tab -= 1;
        self.update()
    }

    pub fn update_budget(&mut self, b: PartialBudget) -> DBResult<()> {
        update_budget(&self.conn, Box::new(b))?;
        self.update()
    }

    pub fn add_new_transaction(&mut self, t: PartialBudgetTransaction, id: u32) -> DBResult<()> {
        add_transaction(&mut self.conn, t, id)?;
        self.update()
    }

    pub fn update_transaction(&mut self, t: BudgetTransaction) -> DBResult<()> {
        update_transaction(&self.conn, t)?;
        self.update()
    }

    pub fn remove_transaction(&mut self, id: u32) -> DBResult<()> {
        remove_transaction(&self.conn, id)?;
        self.update()
    }
}

#[derive(Debug)]
pub enum UserActions {
    Exit,
    Continue,

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

#[derive(Debug)]
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

impl SavableBudget for MinimalBudget {
    fn prepare_for_db(&self) -> MinimalBudget {
        MinimalBudget {
            total: self.total,
            name: self.name.to_owned(),
        }
    }

    fn get_without_transactions(&self) -> PartialBudget {
        PartialBudget {
            id: 0,
            total: self.total,
            name: self.name.to_owned(),
        }
    }
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