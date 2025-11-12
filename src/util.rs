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

impl Word {
    pub fn apply(&self, ws: &mut Vec<Vec<char>>) {
        for ((x, y), ch) in self.positions_and_chars() {
            log::trace!("x: {}, y: {}, place_type: {:?}, ch: {ch}", self.x, self.y, self.place_type);
            ws[y as usize][x as usize] = ch;
        }
    }

    pub fn positions_and_chars(&self) -> Vec<((u8, u8), char)> {
        let mut res: Vec<((u8, u8), char)> = vec![];

        for (i, ch) in self.word.chars().enumerate() {
            match self.place_type {
                PlaceType::UpperLeftDiagonal => {
                    log::trace!("x: {}, y: {}, i: {i}, place_type: {:?}", self.x, self.y, self.place_type);
                    res.push(((self.x - i as u8, self.y - i as u8), ch));
                },
                PlaceType::UpperRightDiagonal => {
                    log::trace!("x: {}, y: {}, i: {i}, place_type: {:?}", self.x, self.y, self.place_type);
                    res.push(((self.x + i as u8, self.y - i as u8), ch));
                },
                PlaceType::LowerRightDiagonal => {
                    log::trace!("x: {}, y: {}, i: {i}, place_type: {:?}", self.x, self.y, self.place_type);
                    res.push(((self.x + i as u8, self.y + i as u8), ch));
                },
                PlaceType::LowerLeftDiagonal => {
                    log::trace!("x: {}, y: {}, i: {i}, place_type: {:?}", self.x, self.y, self.place_type);
                    res.push(((self.x - i as u8, self.y + i as u8), ch));
                },
                PlaceType::RightStraight => {
                    log::trace!("x: {}, y: {}, i: {i}, place_type: {:?}", self.x, self.y, self.place_type);
                    res.push(((self.x + i as u8, self.y), ch));
                },
                PlaceType::UpStraight => {
                    log::trace!("x: {}, y: {}, i: {i}, place_type: {:?}", self.x, self.y, self.place_type);
                    res.push(((self.x, self.y - i as u8), ch));
                },
                PlaceType::LeftStraight => {
                    log::trace!("x: {}, y: {}, i: {i}, place_type: {:?}", self.x, self.y, self.place_type);
                    res.push(((self.x - i as u8, self.y), ch));
                },
                PlaceType::DownStraight => {
                    log::trace!("x: {}, y: {}, i: {i}, place_type: {:?}", self.x, self.y, self.place_type);
                    res.push(((self.x, self.y + i as u8), ch));
                },
            }
        }

        res
    }
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

