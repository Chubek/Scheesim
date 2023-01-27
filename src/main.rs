use std::vec;

use scheesim_solve::EliminatorSolver;

fn main() {   
    let coeff = vec![vec![1.0, 4., 5.], vec![6., 8., 22.], vec![32., 5., 5.]];
    let rhs = vec![1.0, 2.0, 3.0];
    
    let solver = EliminatorSolver::new(&coeff, &rhs);

    let res = solver.factorize_eliminate_solve();

    println!("{res:?}");
}
