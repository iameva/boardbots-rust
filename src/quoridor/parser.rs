extern crate nom;
use nom::{
  branch::alt,
  bytes::complete::{tag, take_while_m_n},
  character::complete::{char, space1},
  combinator::map_res,
  sequence::tuple,
  IResult,
};

use crate::quoridor::{Move, Orientation, Position};

fn is_digit(c: char) -> bool {
  c.is_digit(10)
}

fn coordinate(input: &str) -> IResult<&str, usize> {
  map_res(take_while_m_n(1, 1, is_digit), |s| {
    usize::from_str_radix(s, 10)
  })(input)
}

fn orientation(input: &str) -> IResult<&str, Orientation> {
  let (input, c) = alt((char('h'), char('v')))(input)?;
  match c {
    'h' => Ok((input, Orientation::Horizontal)),
    'v' => Ok((input, Orientation::Vertical)),
    _ => unreachable!(),
  }
}

fn wall_move(input: &str) -> IResult<&str, Move> {
  let (input, (_, _, x, _, y, _, o)) = tuple((
    tag("wall"),
    space1,
    coordinate,
    space1,
    coordinate,
    space1,
    orientation,
  ))(input)?;
  Ok((input, Move::Wall(Position { x, y }, o)))
}

fn pawn_move(input: &str) -> IResult<&str, Move> {
  let (input, (_, _, x, _, y)) =
    tuple((tag("pawn"), space1, coordinate, space1, coordinate))(input)?;
  Ok((input, Move::Pawn(Position { x, y })))
}

pub fn parse_move(input: &str) -> IResult<&str, Move> {
  alt((wall_move, pawn_move))(input)
}
