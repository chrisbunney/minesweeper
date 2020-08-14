use ::minesweeper;
fn main() {
    let mut g = minesweeper::minesweeper::Game::new(10, 10);
    g.run();
}
