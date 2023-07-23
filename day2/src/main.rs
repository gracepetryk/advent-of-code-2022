mod rps;

fn main() {
    let mut game = rps::Game::new();
    game.play_round(rps::Choice::Rock, rps::Choice::Scissors);
    println!("{}, {}", game.p1_score, game.p2_score);
}
