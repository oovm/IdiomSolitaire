use idiom_solitaire::{SolitaireMode, SolitaireSolver};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn one() {
    let mut solver = SolitaireSolver::default();
    solver.load(include_bytes!("../../external/database.csv")).unwrap();
    println!("{:#?}", solver.solve("为所欲为"));
}

#[test]
fn chain() {
    let mut solver = SolitaireSolver::default();
    solver.mode = SolitaireMode::Character;
    solver.load(include_bytes!("../../external/database.csv")).unwrap();
    println!("{:#?}", solver.solve_chain("为所欲为", 100).0);
}
