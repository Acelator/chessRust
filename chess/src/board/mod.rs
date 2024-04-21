#[derive(Copy, Clone)]
pub enum Color {
    White,
    Black
}

#[derive(Copy, Clone)]
pub enum Pieces {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}


pub struct Board {
    pieces: [u64; 6] ,
    player_pieces: [u64; 2],
    
    player_to_move: Color,
    castling_rights: [bool; 2],
    halfmove_clock: u8,

}

impl Board {
    // TODO: Take FEN notation to create a board at a set point. STARTING POS => rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
    // TODO: Change to hexadecimal values
    pub fn new(s: String) -> Board {
        if s.is_empty() {
           Board{pieces: [71776119061282560, 4755801206503243842, 2594073385365405732, 9295429630892703873, 576460752303423504, 1152921504606846984], 
                 player_pieces: [65535, 18446462598732840960], player_to_move: Color::White, castling_rights: [true, true], halfmove_clock: 0} 
        } else {
            // DO stuff
            Board{pieces: [71776119061282560, 4755801206503243842, 2594073385365405732, 9295429630892703873, 576460752303423504, 1152921504606846984], 
               player_pieces: [65535, 18446462598732840960], player_to_move: Color::White, castling_rights: [true, true], halfmove_clock: 0}
        }
    } 

    pub fn get_board(&mut self) -> u64 {
       self.player_pieces[0] | self.player_pieces[1]
    }

    pub fn get_pieces(&mut self, piece: &Pieces, player: Option<&Color>) -> u64 {
        match player {
            Some(c) => self.player_pieces[*c as usize] & self.pieces[*piece as usize],
            None => self.pieces[*piece as usize]
        }
    }

    pub fn get_player_pieces(&mut self, player: &Color, pieces: Option<&Pieces>) -> u64 {
        match pieces {
            Some(p) => self.player_pieces[*player as usize] & self.pieces[*p as usize],
            None => self.player_pieces[*player as usize]
        }
    }
}
