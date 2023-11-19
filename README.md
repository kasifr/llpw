# llp

I need a quick and dirty utility to solve tiny LP problems and come across the promising and awesome rust crate [ellp](https://crates.io/crates/ellp).

I find relevant to write a little wrapper so as to have less bloated code whenever I have to solve a LP problem and come with this.

To illustrate my intent, please see here after the crate's sample code rewritten using the wrapper. Judge by yourself ;-)

👍 to [Kurt Ehlert](https://github.com/kehlert) for providing the crate to the community!


Sample code:
```rust
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
```

Output:

```
5 variables and 3 constraints

minimize
+ 2 x1 + 10 x2 + 1 x4

subject to
+ 2.5 x1 + 3.5 x2 ≥ 5
+ 2.5 x2 + 4.5 x1 ≤ 1
- 1 x3 - 3 x4 - 4 x5 = 2

with the bounds
-1 ≤ x1 ≤ 1
x2 ≤ 6
x3 ≥ 0
x4 = 0
x5 free

Objective value: 19.157894736842103
Optimal point:
  ┌                     ┐
  │ -0.9473684210526313 │
  │  2.1052631578947367 │
  │                   0 │
  │                   0 │
  │                -0.5 │
  └                     ┘


Objective value: 19.157894736842103
Optimal point:
  ┌                     ┐
  │ -0.9473684210526314 │
  │  2.1052631578947367 │
  │                   0 │
  │                   0 │
  │                -0.5 │
  └                     ┘
```
