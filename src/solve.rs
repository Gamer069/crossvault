use crate::util::PlaceType;

pub fn find_word_directionally(grid: &Vec<Vec<char>>, word: &str, reversed: bool) -> bool {
    let search_word = if reversed {
        word.chars().rev().collect::<String>()
    } else {
        word.to_string()
    };

    let first_char = search_word.chars().next().unwrap();
    let indices = crate::util::calculate_indices(grid, first_char);

    for (r, c) in indices {
        let directions = vec![
            (-1, -1, PlaceType::UpperLeftDiagonal),
            (-1, 0, PlaceType::UpStraight),
            (-1, 1, PlaceType::UpperRightDiagonal),
            (0, -1, PlaceType::LeftStraight),
            (0, 1, PlaceType::RightStraight),
            (1, -1, PlaceType::LowerLeftDiagonal),
            (1, 0, PlaceType::DownStraight),
            (1, 1, PlaceType::LowerRightDiagonal)
        ];

        for (dr, dc, place_type) in directions {
            if search_from(&grid, &search_word, r, c, dr, dc) {
                log::info!("Found word: '{word}' starting at ({c}, {r}) going {place_type:?}{}", if reversed { " (reversed)" } else { "" });
                return true;
            }
        }
    }
    false
}

pub fn search_from(grid: &Vec<Vec<char>>, word: &str, r: usize, c: usize, dr: i32, dc: i32) -> bool {
    let mut chars = word.chars();
    let mut r = r as isize;
    let mut c = c as isize;

    while let Some(char_to_find) = chars.next() {
        if r < 0 || c < 0 || r as usize >= grid.len() || c as usize >= grid[0].len() {
            return false;
        }

        let grid_char = grid[r as usize][c as usize];
        if grid_char.to_ascii_lowercase() != char_to_find.to_ascii_lowercase() {
            return false;
        }

        r += dr as isize;
        c += dc as isize;
    }

    true
}


