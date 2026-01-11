use good_lp::{ProblemVariables, Solution, SolverModel, default_solver, variable};
use std::error::Error;

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let mut vars = ProblemVariables::new();
    let x1 = vars.add(variable().min(0).integer());
    let x2 = vars.add(variable().min(0).integer());
    let x3 = vars.add(variable().min(0).integer());
    let x4 = vars.add(variable().min(0).integer());

    let problem = vars.minimise(x1 + x2 + x3 + x4).using(default_solver);

    let solution = problem
        // urban
        .with((-2 * x1 + 8 * x2 + 10 * x4).geq(50))
        // suburban
        .with((5 * x1 + 2 * x2).geq(100))
        //rural
        .with((3 * x1 + -5 * x2 + 10 * x3 - 2 * x4).geq(25))
        .solve()?;

    println!(
        "a={:?}",
        vec![
            solution.value(x1),
            solution.value(x2),
            solution.value(x3),
            solution.value(x4)
        ]
    );

    Ok(())
}
