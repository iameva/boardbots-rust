/// One-indexed player number
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub struct Player(pub usize);
pub trait Board: Sized {
  type Move;
  type Outcome;

  fn new(players: usize) -> Option<Self>;
  fn is_valid(&self, mv: &Self::Move) -> bool;
  fn make_move(&mut self, mv: &Self::Move) -> bool;
  fn next_player(&self) -> Option<Player>;
  fn result(&self) -> Option<Self::Outcome>;
}
