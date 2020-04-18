#![feature(map_first_last)]
mod games;
mod quoridor;

use self::quoridor::*;
use games::Board;
use parser::parse_move;
use printer::print_quoridor;
use std::io::{self, Read, Write};

fn main() {
  let mut game: Quoridor = games::Board::new(2).unwrap();

  let mut quit: bool = false;
  while !quit {
    println!("{:?}\n{}", game, print_quoridor(&game));
    let mut buffer = String::new();
    print!("\n> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    println!("{}", buffer);
    match buffer.trim() {
      "exit" | "quit" | "q" => {
        quit = true;
      }
      line => match parse_move(line) {
        Ok((_, m)) => {
          println!("move: {:?}", m);
          if !game.make_move(&m) {
            println!("Invalid Move! try again.");
          }
        }
        error => {
          println!("{:?}", error);
        }
      },
    }
  }

  return;
}
