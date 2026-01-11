use good_lp::{Solution, SolverModel, default_solver, variable, variables};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    variables! {
        vars:
        a <= 1;
        2 <= b <= 4;
    }

    // ... or add variables programmatically
    vars.add(variable().min(2).max(9));

    let solution = vars
        .maximise(10 * (a - b / 5) - b)
        .using(default_solver)
        .with(a + 2. << b) // or (a + 2).leq(b)
        .with(1 + a >> 4. - b)
        .solve()?;

    println!("a={:?}, b={:?}", solution.value(a), solution.value(b));

    Ok(())
}
