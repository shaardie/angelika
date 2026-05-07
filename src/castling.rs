#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum Castling {
    WhiteKingSite = 0b0001,
    WhiteQueenSite = 0b0010,
    BlackKingSite = 0b0100,
    BlackQueenSite = 0b1000,
    None = 0b0000,
    Any = 0b1111,
}
