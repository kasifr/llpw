mod lpp; 
use lpp::LinearProgrammingProblem;

use ellp::*;

fn main() {
    let mut lp_problem = LinearProgrammingProblem::new();

    let x1 = lp_problem.add_variable(2., Bound::TwoSided(-1., 1.), "x1");
    let x2 = lp_problem.add_variable(10., Bound::Upper(6.), "x2");
    let x3 = lp_problem.add_variable(0., Bound::Lower(0.), "x3");
    let x4 = lp_problem.add_variable(1., Bound::Fixed(0.), "x4");
    let x5 = lp_problem.add_variable(0., Bound::Free, "x5");

    lp_problem.add_constraint(vec![(x1, 2.5), (x2, 3.5)], ConstraintOp::Gte, 5.);
    lp_problem.add_constraint(vec![(x2, 2.5), (x1, 4.5)], ConstraintOp::Lte, 1.);
    lp_problem.add_constraint(vec![(x3, -1.), (x4, -3.), (x5, -4.)], ConstraintOp::Eq, 2.);

    println!("{}", lp_problem);

    lp_problem.solve_primal_and_print();
    lp_problem.solve_dual_and_print();
}
