use good_lp::{Solution, SolverModel, variable, ProblemVariables};
use good_lp::solvers::lpsolve;
use std::error::Error;

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let mut vars = ProblemVariables::new();
    let a = vars.add(variable().max(9));
    let b = vars.add(variable().min(2).max(4));

    let problem = vars
        .maximise(10 * (a - b / 5) - b)
        .using(lpsolve::lp_solve);
  
    let solution = problem
        .with((a + 2).leq(b))
        .with((1 + a).geq(4. - b))
        .solve()?;

    println!("a={:?}, b={:?}", solution.value(a), solution.value(b));

    Ok(())
}
