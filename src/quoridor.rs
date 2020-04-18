pub mod parser;
pub mod printer;

use crate::games;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug)]
pub struct Player {
  pub position: Position,
  pub walls_remaining: u8,
  pub player: games::Player,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum Orientation {
  Horizontal,
  Vertical,
}

/// Represents a position on the Quoridor board. 1-9 are valid.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct Position {
  pub x: usize,
  pub y: usize,
}

#[derive(Debug)]
pub struct Wall {
  pub player: games::Player,
  pub position: Position,
  pub orientation: Orientation,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Move {
  Pawn(Position),
  Wall(Position, Orientation),
}

#[derive(Debug)]
pub struct Quoridor {
  pub players: Vec<Player>,
  pub walls: Vec<Wall>,
  pub turns: usize,
}

pub enum Outcome {
  Draw,
  Winner(Player),
}

impl games::Board for Quoridor {
  type Move = self::Move;
  type Outcome = self::Outcome;

  fn is_valid(&self, mv: &Move) -> bool {
    true
  }

  fn make_move(&mut self, mv: &Move) -> bool {
    if self.is_valid(mv) {
      let len = self.players.len();
      let mut player = self.players.get_mut(self.turns % len).expect("img!");
      match mv {
        Move::Pawn(pos) => {
          player.position = *pos;
        }
        Move::Wall(pos, or) => {
          player.walls_remaining -= 1;
          self.walls.push(Wall {
            player: player.player,
            position: *pos,
            orientation: *or,
          });
        }
      };
      self.turns += 1;
      true
    } else {
      false
    }
  }

  fn new(players: usize) -> Option<Quoridor> {
    if players == 2 {
      Some(Quoridor {
        players: vec![
          Player {
            position: Position { x: 5, y: 1 },
            walls_remaining: 10,
            player: games::Player(1),
          },
          Player {
            position: Position { x: 5, y: 9 },
            walls_remaining: 10,
            player: games::Player(2),
          },
        ],
        walls: vec![],
        turns: 0,
      })
    } else if players == 4 {
      Some(Quoridor {
        players: vec![
          Player {
            position: Position { x: 5, y: 9 },
            walls_remaining: 5,
            player: games::Player(1),
          },
          Player {
            position: Position { x: 1, y: 5 },
            walls_remaining: 5,
            player: games::Player(2),
          },
          Player {
            position: Position { x: 9, y: 5 },
            walls_remaining: 5,
            player: games::Player(3),
          },
          Player {
            position: Position { x: 5, y: 1 },
            walls_remaining: 5,
            player: games::Player(4),
          },
        ],
        walls: vec![],
        turns: 0,
      })
    } else {
      None
    }
  }

  fn next_player(&self) -> Option<games::Player> {
    let num_players = self.players.len();
    self.players.get(self.turns % num_players).map(|p| p.player)
  }

  fn result(&self) -> Option<Outcome> {
    None
  }
}
