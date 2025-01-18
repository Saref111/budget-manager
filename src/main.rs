use types::Budget;
use ui::run_ui;

mod types;
mod  ui;

fn main() {
    let budget = Budget::new(None, None, 1000);

    let ui = run_ui();
}
