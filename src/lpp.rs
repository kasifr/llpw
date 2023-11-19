use std::fmt;

use ellp::*;
use ellp::problem::{VariableId, Problem};

pub struct LinearProgrammingProblem {
    prob: Problem,

    primal_solver: PrimalSimplexSolver,
    dual_solver: DualSimplexSolver,

    primal_result: Option<SolverResult>,
    dual_result: Option<SolverResult>,
}


impl LinearProgrammingProblem {
    pub fn new() -> Self {
        LinearProgrammingProblem {
            prob: Problem::new(),
            primal_solver: PrimalSimplexSolver::default(),
            dual_solver: DualSimplexSolver::default(),
            primal_result: None,
            dual_result: None,
        }
    }

    pub fn add_variable(&mut self, coefficient: f64, bound: Bound, name: &str) -> VariableId {
        self.prob.add_var(coefficient, bound, Some(name.to_string())).unwrap()
    }

    pub fn add_constraint(&mut self, variables: Vec<(VariableId, f64)>, op: ConstraintOp, rhs: f64) {
        self.prob.add_constraint(variables, op, rhs).unwrap();
    }

    pub fn solve_dual_and_print(&mut self) {
        self.dual_result = Some(self.dual_solver.solve(self.prob.clone()).unwrap());
        self.print_solution(&self.dual_result.as_ref().unwrap());
    }

    pub fn solve_primal_and_print(&mut self) {
        self.primal_result = Some(self.primal_solver.solve(self.prob.clone()).unwrap());
        self.print_solution(&self.primal_result.as_ref().unwrap());
    }

    fn print_solution(&self, result: &SolverResult) {
        match result {
            SolverResult::Optimal(sol) => {
                println!("Objective value: {}", sol.obj());
                println!("Optimal point: {}", sol.x());
            }
            _ => panic!("Should have an optimal point"),
        }
    }
}

impl fmt::Display for LinearProgrammingProblem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.prob.fmt(f)
    }
}
