use crate::storage::typedef::Playable;

pub fn run_game<G, F>(mut game: G, mut get_input: F)
where
    G: Playable,
    F: FnMut(&G) -> G::Action,
{
    while !game.is_over() {
        // 1. Get input (from CLI, GUI, or AI)
        let action = get_input(&game);

        // 2. Execute turn
        game.step_turn(action);
    }

    println!("{}", game.winner());
}