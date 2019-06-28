pub mod position;

trait Piece {
	fn get_position(&self);
    fn get_possible_movements(&self) -> [position::Position];
    /*pub fn r#move(&self, _: u8) -> Result<position::Position, position::Position> {

    }*/
}

fn set_piece_position(piece: &impl Piece, position: position::Position){
	piece.get_position();
}