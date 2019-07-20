#![allow(unused_imports)]

use std::convert::TryFrom;
use std::fmt::Debug;

use chess::{Board, BoardStatus, ChessMove, Color, Game, GameResult, MoveGen};
use rand::{seq::SliceRandom, thread_rng, Rng};
use shakmaty as shak_chess;

fn main() {
    let white = MoveForward;
    let black = RandomAgent;

    play_n_games(100, MoveForward, RandomAgent);
    play_n_games(100, MoveBackward, RandomAgent);
    play_n_games(100, MoveForward, MoveBackward);
}
/// Plays N games between two agents, alternating sides, and returns the
/// fraction which were won by player one.
pub fn play_n_games(n: u32, player_one: impl Agent, player_two: impl Agent) -> f64 {
    let mut player_one_wins = 0.0;
    for i in 0..n {
        let player_one = player_one.clone();
        let player_two = player_two.clone();

        use chess::GameResult::*;
        player_one_wins += if i % 2 == 0 {
            match play_game(player_one, player_two) {
                WhiteCheckmates => 1.0,
                WhiteResigns => 0.0,
                BlackCheckmates => 0.0,
                BlackResigns => 1.0,
                Stalemate => 0.5,
                DrawAccepted => 0.5,
                DrawDeclared => 0.5,
            }
        } else {
            match play_game(player_two, player_one) {
                WhiteCheckmates => 0.0,
                WhiteResigns => 1.0,
                BlackCheckmates => 1.0,
                BlackResigns => 0.0,
                Stalemate => 0.5,
                DrawAccepted => 0.5,
                DrawDeclared => 0.5,
            }
        }
    }
    let ratio = player_one_wins / f64::from(n);

    println!(
        "{:?} vs {:?}: {:?} won {:.1}%",
        &player_one,
        &player_two,
        &player_one,
        ratio * 100.0
    );

    ratio
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

    game.result().expect("game must a result")
}

pub trait Agent: Debug + Clone {
    fn pick_move<'moves>(
        &mut self,
        board: &Board,
        moves: Vec<&'moves ChessMove>,
    ) -> &'moves ChessMove;
}

pub trait RankMove: Debug + Clone {
    fn rank_move(&mut self, board: &Board, chess_move: &ChessMove) -> i64;
}

impl<T> Agent for T
where
    T: RankMove,
{
    fn pick_move<'moves>(
        &mut self,
        board: &Board,
        mut moves: Vec<&'moves ChessMove>,
    ) -> &'moves ChessMove {
        moves.shuffle(&mut thread_rng());
        moves.sort_by_cached_key(|candidate| {
            (
                self.rank_move(&board, &candidate),
                thread_rng().gen::<i64>(),
            )
        });
        moves[moves.len() - 1]
    }
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

#[derive(Clone, Copy, Debug)]
struct MoveForward;

impl RankMove for MoveForward {
    fn rank_move(&mut self, board: &Board, chess_move: &ChessMove) -> i64 {
        let my_color = board.side_to_move();
        let rank_shift = i64::try_from(chess_move.get_dest().get_rank().to_index()).unwrap()
            - i64::try_from(chess_move.get_source().get_rank().to_index()).unwrap();
        let forward_shift = if my_color == Color::White {
            rank_shift
        } else {
            -rank_shift
        };
        forward_shift
    }
}

#[derive(Clone, Copy, Debug)]
struct MoveBackward;

impl RankMove for MoveBackward {
    fn rank_move(&mut self, board: &Board, chess_move: &ChessMove) -> i64 {
        let my_color = board.side_to_move();
        let rank_shift = i64::try_from(chess_move.get_dest().get_rank().to_index()).unwrap()
            - i64::try_from(chess_move.get_source().get_rank().to_index()).unwrap();
        let backward_shift = if my_color == Color::White {
            -rank_shift
        } else {
            rank_shift
        };
        backward_shift
    }
}
