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
            PlaceType::UpStraight => true,
            PlaceType::DownStraight => true,
            _ => false
        }
    }

    pub fn is_horizontal(&self) -> bool {
        match self {
            PlaceType::RightStraight => true,
            PlaceType::LeftStraight => true,
            _ => false
        }
    }

    pub fn is_diagonal(&self) -> bool {
        match self {
            PlaceType::RightStraight => false,
            PlaceType::UpStraight => false,
            PlaceType::DownStraight => false,
            _ => true
        }
    }

    pub fn is_reversed(&self) -> bool {
        match self {
            PlaceType::LeftStraight => true,
            _ => false
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

pub fn calculate_indices(grid: &Vec<Vec<char>>, target: char) -> Vec<(usize, usize)> {
    grid.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(j, &c)| {
                    if c.to_ascii_lowercase() == target.to_ascii_lowercase() {
                        Some((i, j))
                    } else {
                        None
                    }
                })
        })
    .collect()
}

