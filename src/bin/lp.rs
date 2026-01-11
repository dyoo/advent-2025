use good_lp::{
    Expression, ProblemVariables, Solution, SolverModel, Variable, default_solver, variable,
};
use std::error::Error;

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let mut problem_variables = ProblemVariables::new();
    let vars: Vec<Variable> = problem_variables.add_all(vec![variable().min(0).integer(); 4]);

    let goal = vars.iter().sum::<Expression>();
    let problem = problem_variables.minimise(goal).using(default_solver);

    let data = [
        ([-2, 8, 0, 10], 50),
        ([5, 2, 0, 0], 100),
        ([3, -5, 10, -2], 25),
    ];
    let constraints = data
        .iter()
        .map(|(vals, goal)| {
            vals.iter()
                .zip(vars.iter())
                .map(|(&x, &y)| x * y)
                .sum::<Expression>()
                .geq(*goal)
        })
        .collect::<Vec<_>>();

    println!("{:?}", constraints);

    let solution = problem.with_all(constraints).solve()?;
    println!(
        "{:?}",
        vars.iter()
            .map(|var| solution.value(*var))
            .collect::<Vec<_>>()
    );

    Ok(())
}
