extern crate itertools;

use itertools::Itertools;


fn brute_force(maxsize: u64, pieces: Vec<Piece>) -> u64 {
    let mut best_sum = 0;
    for comb_size in 1..pieces.len() {
        for combination in pieces.iter().combinations(comb_size) {
            let weight_sum: u64 = combination.iter().map(|piece| piece.weight).sum();
            let value_sum: u64 = combination.iter().map(|piece| piece.value).sum();
            if value_sum > best_sum && weight_sum <= maxsize {
                best_sum = value_sum
            }
        }
    }
    best_sum
}


#[derive(Debug)]
struct Piece {
    weight: u64,
    value: u64
}

fn main() {
    let maxsize = 15;
    let pieces = vec![
        Piece{value: 4, weight: 12},
        Piece{value: 2, weight: 1},
        Piece{value: 6, weight: 4},
        Piece{value: 1, weight: 1},
        Piece{value: 2, weight: 2},
    ];
    println!("{}", brute_force(maxsize, pieces));
}
