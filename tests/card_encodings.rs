use rustsat::{
    clause,
    encodings::{
        card::{EncodeCard, IncEncodeCard, Totalizer},
        BoundType,
    },
    instances::{BasicVarManager, ManageVars},
    lit,
    solvers::{ipasir::IpasirSolver, IncrementalSolve, Solve, SolverResult},
    types::{Clause, Lit, Var},
    var,
};

/// Requires an incremental cardinality encoding with upper and lower bounding functionality
fn test_card_pos_lits<CE: IncEncodeCard>(mut enc: CE) {
    // Set up instance
    let mut solver = IpasirSolver::new();
    solver.add_clause(clause![lit![0], lit![1]]);
    solver.add_clause(clause![lit![1]]);
    solver.add_clause(clause![lit![1], lit![2]]);
    solver.add_clause(clause![lit![2], lit![3]]);
    solver.add_clause(clause![lit![3], lit![4]]);
    solver.add_clause(clause![lit![4]]);
    solver.add_clause(clause![lit![5]]);
    solver.add_clause(clause![lit![6], lit![7]]);
    solver.add_clause(clause![lit![7]]);
    solver.add_clause(clause![lit![7], lit![8]]);
    solver.add_clause(clause![lit![8], lit![9]]);
    solver.add_clause(clause![lit![9], lit![10]]);
    solver.add_clause(clause![lit![10]]);
    let mut var_manager = BasicVarManager::new();
    var_manager.increase_next_free(var![11]);

    let res = solver.solve().unwrap();
    assert_eq!(res, SolverResult::SAT);

    enc.add(vec![lit![0], lit![1], lit![2], lit![3], lit![4]]);

    enc.encode(2, 2, &mut var_manager).add_to_solver(&mut solver);
    let assumps = enc.enforce_lb(2).unwrap();
    let res = solver.solve_assumps(assumps).unwrap();
    assert_eq!(res, SolverResult::SAT);

    let assumps = enc.enforce_ub(2).unwrap();
    let res = solver.solve_assumps(assumps).unwrap();
    assert_eq!(res, SolverResult::UNSAT);

    enc.encode_change(0, 3, &mut var_manager)
        .add_to_solver(&mut solver);
    let assumps = enc.enforce_ub(3).unwrap();
    let res = solver.solve_assumps(assumps).unwrap();
    assert_eq!(res, SolverResult::SAT);

    enc.add(vec![lit![5]]);

    enc.encode_change(0, 3, &mut var_manager)
        .add_to_solver(&mut solver);
    let assumps = enc.enforce_ub(3).unwrap();
    let res = solver.solve_assumps(assumps).unwrap();
    assert_eq!(res, SolverResult::UNSAT);

    enc.encode_change(0, 4, &mut var_manager)
        .add_to_solver(&mut solver);
    let assumps = enc.enforce_ub(4).unwrap();
    let res = solver.solve_assumps(assumps).unwrap();
    assert_eq!(res, SolverResult::SAT);

    enc.add(vec![lit![6], lit![7], lit![8], lit![9], lit![10]]);

    enc.encode_change(0, 4, &mut var_manager)
        .add_to_solver(&mut solver);
    let assumps = enc.enforce_ub(4).unwrap();
    let res = solver.solve_assumps(assumps).unwrap();
    assert_eq!(res, SolverResult::UNSAT);

    enc.encode_change(0, 7, &mut var_manager)
        .add_to_solver(&mut solver);
    let assumps = enc.enforce_ub(7).unwrap();
    let res = solver.solve_assumps(assumps).unwrap();
    assert_eq!(res, SolverResult::SAT);
}

/// Requires a cardinality encoding with upper and lower bounding functionality
fn test_card_neg_lits<CE: EncodeCard>(mut enc: CE) {
    // Set up instance
    let mut solver = IpasirSolver::new();
    solver.add_clause(clause![lit![0], lit![1]]);
    solver.add_clause(clause![lit![1]]);
    solver.add_clause(clause![lit![1], lit![2]]);
    solver.add_clause(clause![lit![2], lit![3]]);
    solver.add_clause(clause![lit![3], lit![4]]);
    solver.add_clause(clause![lit![4]]);
    let mut var_manager = BasicVarManager::new();
    var_manager.increase_next_free(var![5]);

    let res = solver.solve().unwrap();
    assert_eq!(res, SolverResult::SAT);

    // Set up totalizer
    enc.add(vec![!lit![0], !lit![1], !lit![2], !lit![3], !lit![4]]);

    enc.encode(2, 3, &mut var_manager)
        .add_to_solver(&mut solver);
    let assumps = enc.enforce_ub(2).unwrap();
    let res = solver.solve_assumps(assumps).unwrap();
    assert_eq!(res, SolverResult::SAT);

    let assumps = enc.enforce_lb(3).unwrap();
    let res = solver.solve_assumps(assumps).unwrap();
    assert_eq!(res, SolverResult::UNSAT);

    let assumps = enc.enforce_lb(2).unwrap();
    let res = solver.solve_assumps(assumps).unwrap();
    assert_eq!(res, SolverResult::SAT);
}

/// Requires a cardinality encoding with upper and lower bounding functionality
fn test_card_min_enc<CE: EncodeCard>(mut enc: CE) {
    // Set up instance
    let mut solver = IpasirSolver::new();
    let mut var_manager = BasicVarManager::new();
    var_manager.increase_next_free(var![4]);

    enc.add(vec![lit![0], lit![1], lit![2], lit![3]]);

    enc.encode(3, 3, &mut var_manager)
        .add_to_solver(&mut solver);
    let mut assumps = enc.enforce_eq(3).unwrap();
    assumps.extend(vec![lit![0], lit![1], lit![2], !lit![3]]);
    let res = solver.solve_assumps(assumps).unwrap();
    assert_eq!(res, SolverResult::SAT);

    let mut assumps = enc.enforce_eq(3).unwrap();
    assumps.extend(vec![lit![0], lit![1], !lit![2], lit![3]]);
    let res = solver.solve_assumps(assumps).unwrap();
    assert_eq!(res, SolverResult::SAT);

    let mut assumps = enc.enforce_eq(3).unwrap();
    assumps.extend(vec![lit![0], !lit![1], lit![2], lit![3]]);
    let res = solver.solve_assumps(assumps).unwrap();
    assert_eq!(res, SolverResult::SAT);

    let mut assumps = enc.enforce_eq(3).unwrap();
    assumps.extend(vec![!lit![0], lit![1], lit![2], lit![3]]);
    let res = solver.solve_assumps(assumps).unwrap();
    assert_eq!(res, SolverResult::SAT);

    let mut assumps = enc.enforce_eq(3).unwrap();
    assumps.extend(vec![!lit![0], !lit![1], lit![2], lit![3]]);
    let res = solver.solve_assumps(assumps).unwrap();
    assert_eq!(res, SolverResult::UNSAT);

    let mut assumps = enc.enforce_eq(3).unwrap();
    assumps.extend(vec![!lit![0], lit![1], !lit![2], lit![3]]);
    let res = solver.solve_assumps(assumps).unwrap();
    assert_eq!(res, SolverResult::UNSAT);

    let mut assumps = enc.enforce_eq(3).unwrap();
    assumps.extend(vec![!lit![0], lit![1], lit![2], !lit![3]]);
    let res = solver.solve_assumps(assumps).unwrap();
    assert_eq!(res, SolverResult::UNSAT);

    let mut assumps = enc.enforce_eq(3).unwrap();
    assumps.extend(vec![lit![0], !lit![1], !lit![2], lit![3]]);
    let res = solver.solve_assumps(assumps).unwrap();
    assert_eq!(res, SolverResult::UNSAT);

    let mut assumps = enc.enforce_eq(3).unwrap();
    assumps.extend(vec![lit![0], !lit![1], lit![2], !lit![3]]);
    let res = solver.solve_assumps(assumps).unwrap();
    assert_eq!(res, SolverResult::UNSAT);

    let mut assumps = enc.enforce_eq(3).unwrap();
    assumps.extend(vec![lit![0], lit![1], !lit![2], !lit![3]]);
    let res = solver.solve_assumps(assumps).unwrap();
    assert_eq!(res, SolverResult::UNSAT);
}

#[test]
fn tot_positive_lits() {
    let tot = Totalizer::new(BoundType::BOTH).unwrap();
    test_card_pos_lits(tot);
}

#[test]
fn tot_negative_lits() {
    let tot = Totalizer::new(BoundType::BOTH).unwrap();
    test_card_neg_lits(tot);
}

#[test]
fn tot_min_enc() {
    let tot = Totalizer::new(BoundType::BOTH).unwrap();
    test_card_min_enc(tot);
}