use types::Budget;
use ui::run_ui;

mod types;
mod  ui;

fn main() {
    let budget = Budget::new(None, None, 1000);
    let b1 = Budget::new(Some("Secondary budget".to_string()), None, 123);
    let b2 = Budget::new(Some("Third budget".to_string()), None, 123432);

    let ui = run_ui(vec![budget, b1, b2]);
}
