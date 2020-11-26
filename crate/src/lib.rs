use holdem_hand_evaluator::{heads_up_win_frequency, Hand};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compute_win_rate(used_cards: Box<[i32]>) -> Box<[u32]> {
    let hand1 = Hand::from_slice(&[used_cards[0] as usize, used_cards[1] as usize]);
    let mut hand2 = Hand::new();
    for i in 2..4 {
        if used_cards[i] != -1 {
            hand2 = hand2.add_card(used_cards[i] as usize);
        }
    }
    let mut board = Hand::new();
    for i in 4..9 {
        if used_cards[i] != -1 {
            board = board.add_card(used_cards[i] as usize);
        }
    }
    let win_freq = heads_up_win_frequency(&hand1, &hand2, &board, &Hand::new());
    Box::new([win_freq.0, win_freq.1, win_freq.2])
}
