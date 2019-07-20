#![allow(unused_imports)]
use chess::{Board, ChessMove};
use rand::{seq::SliceRandom, thread_rng};
use shakmaty as shak_chess;

fn main() {
    let board = Board::default();

    println!("{:?}", &board);
}

trait Agent {
    fn pick_move<'moves>(
        &mut self,
        board: Board,
        moves: Vec<&'moves ChessMove>,
    ) -> &'moves ChessMove;
}

struct RandomAgent;
impl Agent for RandomAgent {
    fn pick_move<'moves>(
        &mut self,
        board: Board,
        mut moves: Vec<&'moves ChessMove>,
    ) -> &'moves ChessMove {
        moves.shuffle(&mut thread_rng());
        moves[0]
    }
}
