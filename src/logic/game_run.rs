use crate::storage::typedef::Playable;

pub fn run_game<G, F>(mut game: G, mut get_input: F)
where
    G: Playable,
    F: FnMut(&G) -> G::Action,
{
    while !game.is_over() {
        let action = get_input(&game);
        game.step_turn(action);
    }

    println!("{}", game.winner());
}
