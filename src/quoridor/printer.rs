use crate::quoridor::{Orientation, Quoridor};
use std::cmp::{Ord, Ordering};
use std::collections::BTreeMap;
use std::fmt::Write;

#[derive(Eq, PartialEq, PartialOrd, Debug)]
struct PrintPosition {
  x: usize,
  y: usize,
}

impl Ord for PrintPosition {
  fn cmp(&self, other: &Self) -> Ordering {
    (self.y, self.x).cmp(&(other.y, other.x))
  }
}

fn pp(x: usize, y: usize) -> PrintPosition {
  PrintPosition { x, y }
}

pub fn print_quoridor(board: &Quoridor) -> String {
  let mut positions = BTreeMap::new();
  let mut result = String::new();

  for player in board.players.iter() {
    let pos = pp(player.position.x * 2 + 1, player.position.y * 2 + 1);
    positions.insert(pos, '\u{25CF}');
  }

  for wall in board.walls.iter() {
    match wall.orientation {
      Orientation::Horizontal => {
        positions.insert(pp(wall.position.x * 2 + 1, wall.position.y * 2), '\u{2501}');
        positions.insert(pp(wall.position.x * 2 + 3, wall.position.y * 2), '\u{2501}');
      }
      Orientation::Vertical => {
        positions.insert(pp(wall.position.x * 2, wall.position.y * 2 + 1), '\u{2503}');
        positions.insert(pp(wall.position.x * 2, wall.position.y * 2 + 3), '\u{2503}');
      }
    };
  }

  let mut next_pos = positions.first_entry();

  for y in 0..20 {
    for x in 0..20 {
      match next_pos {
        Some(entry) if *entry.key() == PrintPosition { x, y } => {
          let (_, c) = entry.remove_entry();
          next_pos = positions.first_entry();
          write!(&mut result, "{}", c);
        }
        _ => {
          write!(&mut result, " ");
        }
      }
      write!(&mut result, " ");
    }
    write!(&mut result, "\n");
  }

  result
}
