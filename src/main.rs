#![allow(unused_imports)]

use std::fmt::Debug;

use chess::{Board, BoardStatus, ChessMove, Color, Game, GameResult, MoveGen};
use rand::{seq::SliceRandom, thread_rng};
use shakmaty as shak_chess;

fn main() {
    let board = Board::default();

    for _ in 0..16 {
        let white = RandomAgent;
        let black = RandomAgent;

        println!(
            "{:?} vs {:?}: {:?}",
            &white,
            &black,
            play_game(white, black)
        );
    }
}

pub fn play_game(mut white: impl Agent, mut black: impl Agent) -> GameResult {
    let mut game = Game::new();

    while game.result().is_none() {
        if game.can_declare_draw() {
            game.declare_draw();
            break;
        }

        let board = game.current_position();

        let moves: Vec<ChessMove> = MoveGen::new_legal(&board).collect();
        let move_refs: Vec<&ChessMove> = moves.iter().collect();

        let chosen_move = if board.side_to_move() == Color::White {
            white.pick_move(&board, move_refs)
        } else {
            black.pick_move(&board, move_refs)
        };

        assert!(game.make_move(*chosen_move), "illegal move?");
    }

    game.result().unwrap()
}

pub trait Agent: Debug + Clone {
    fn pick_move<'moves>(
        &mut self,
        board: &Board,
        moves: Vec<&'moves ChessMove>,
    ) -> &'moves ChessMove;
}

#[derive(Clone, Copy, Debug)]
struct RandomAgent;

impl Agent for RandomAgent {
    fn pick_move<'moves>(
        &mut self,
        board: &Board,
        mut moves: Vec<&'moves ChessMove>,
    ) -> &'moves ChessMove {
        moves.shuffle(&mut thread_rng());
        moves[0]
    }
}
