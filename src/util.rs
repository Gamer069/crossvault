use enum_derived::Rand;

#[derive(Clone, Copy, Debug, Rand)]
pub enum PlaceType {
    UpperLeftDiagonal,
    UpperRightDiagonal,

    LowerRightDiagonal,
    LowerLeftDiagonal,

    RightStraight,
    UpStraight,
    LeftStraight,
    DownStraight
}

impl PlaceType {
    pub fn is_vertical(&self) -> bool {
        match self {
            PlaceType::UpperLeftDiagonal => false,
            PlaceType::UpperRightDiagonal => false,
            PlaceType::LowerRightDiagonal => false,
            PlaceType::LowerLeftDiagonal => false,
            PlaceType::RightStraight => false,
            PlaceType::UpStraight => true,
            PlaceType::LeftStraight => false,
            PlaceType::DownStraight => true,
        }
    }

    pub fn is_horizontal(&self) -> bool {
        match self {
            PlaceType::UpperLeftDiagonal => false,
            PlaceType::UpperRightDiagonal => false,
            PlaceType::LowerRightDiagonal => false,
            PlaceType::LowerLeftDiagonal => false,
            PlaceType::RightStraight => true,
            PlaceType::UpStraight => false,
            PlaceType::LeftStraight => true,
            PlaceType::DownStraight => false,
        }
    }

    pub fn is_diagonal(&self) -> bool {
        match self {
            PlaceType::UpperLeftDiagonal => true,
            PlaceType::UpperRightDiagonal => true,
            PlaceType::LowerRightDiagonal => true,
            PlaceType::LowerLeftDiagonal => true,
            PlaceType::RightStraight => false,
            PlaceType::UpStraight => false,
            PlaceType::LeftStraight => true,
            PlaceType::DownStraight => false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Word {
    pub place_type: PlaceType,
    pub word: String,
    pub x: u8,
    pub y: u8,
}
