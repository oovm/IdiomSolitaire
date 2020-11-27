use idiom_solitaire::{SolitaireMode, SolitaireSolver};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn one() {
    let mut solver = SolitaireSolver::default();
    solver.load(include_bytes!("../../external/database.csv")).unwrap();
    println!("{:#?}", solver.solve_random("为所欲为"));
}

#[test]
fn chain() {
    let mut solver = SolitaireSolver::default();
    solver.mode = SolitaireMode::Sound;
    solver.load(include_bytes!("../../external/database.csv")).unwrap();
    // println!("{:#?}",solver.dict.sound_map().iter().next().unwrap());
    println!("{:#?}", solver.solve_chain("耗子尾汁", 100));
}

#[test]
fn search() {
    let mut solver = SolitaireSolver::default();
    solver.mode = SolitaireMode::Character;
    solver.load(include_bytes!("../../external/database.csv")).unwrap();
    // println!("{:#?}",solver.dict.sound_map().iter().next().unwrap());
    println!("{:#?}", solver.solve_target("为所欲为", "耗子尾汁"));
}
