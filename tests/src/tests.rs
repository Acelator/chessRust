use std::string;

use chess;
use chess::board::*;

#[test]
fn initial_board() {
    let s = String::new();

    let mut board = Board::new(s);
    assert_eq!(0xffff00000000ffff, board.get_board());
    assert_eq!(0xff00000000ff00, board.get_pieces(&Pieces::Pawn, None));
    assert_eq!(0x8, board.get_pieces(&Pieces::King, Some(&Color::White)));

    assert_eq!(0xffff000000000000, board.get_player_pieces(&Color::Black, None));
    assert_eq!(0x81, board.get_player_pieces(&Color::White, Some(&Pieces::Rook)));
}
