use std::env;

mod parse_input;
mod rps;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];
    let mut choice_reader = parse_input::ChoiceReader::new(input_path).unwrap();

    let mut game = rps::Game::new();

    for round in &mut choice_reader {
        let (opponent_choice, desired_outcome) = match round {
            Ok((opponent_choice, desired_outcome)) => (opponent_choice, desired_outcome),
            Err(e) => panic!("{}", e),
        };

        let player_choice = opponent_choice.fix_game(desired_outcome);

        println!("{desired_outcome:?}, {player_choice:?}, {opponent_choice:?}");
        game.play_round(player_choice, opponent_choice);
    }

    println!("player: {}, opponent: {}, rounds: {}", game.p1_score, game.p2_score, choice_reader.rounds_read)

}
