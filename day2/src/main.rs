use std::env;

mod parse_input;
mod rps;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];
    let mut choice_reader = parse_input::ChoiceReader::new(input_path).unwrap();

    let mut game = rps::Game::new();

    for round in &mut choice_reader {
        let (opponent_choice, player_choice) = match round {
            Ok((opponent_choice, player_choice)) => (opponent_choice, player_choice),
            Err(e) => panic!("{}", e),
        };

        println!("{player_choice:?}, {opponent_choice:?}");

        game.play_round(player_choice, opponent_choice);
        println!("")
    }

    println!("player: {}, opponent: {}, rounds: {}", game.p1_score, game.p2_score, choice_reader.rounds_read)

}
