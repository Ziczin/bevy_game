/// Универсальный маппинг для 47-тайлового blob tileset.

const fn build_blob_map() -> [u8; 256] {
    let mut map = [0u8; 256];
    let mut next_index = 0u8;
    let mut i = 0usize;
    while i < 256 {
        let n  = ((i >> 6) & 1) as u8;
        let e  = ((i >> 4) & 1) as u8;
        let s  = ((i >> 2) & 1) as u8;
        let w  = ( i       & 1) as u8;

        let nw = (((i >> 7) & 1) as u8) & n & w;
        let ne = (((i >> 5) & 1) as u8) & n & e;
        let se = (((i >> 3) & 1) as u8) & s & e;
        let sw = (((i >> 1) & 1) as u8) & s & w;

        let canonical = ((nw as usize) << 7)
                      | ((n  as usize) << 6)
                      | ((ne as usize) << 5)
                      | ((e  as usize) << 4)
                      | ((se as usize) << 3)
                      | ((s  as usize) << 2)
                      | ((sw as usize) << 1)
                      |  (w  as usize);

        if canonical == i {
            map[i] = next_index;
            next_index += 1;
        } else {
            map[i] = map[canonical];
        }
        i += 1;
    }
    map
}

pub const BLOB_MAP: [u8; 256] = build_blob_map();

pub const ATLAS_COLS: usize = 12;
#[allow(dead_code)]
pub const ATLAS_ROWS: usize = 4;

macro_rules! remap {
    ( $( $id:literal => ($col:literal, $row:literal) ),* $(,)? ) => {
        [ $( ($col as u8, $row as u8) ),* ]
    };
}

/// REMAP: id (0..46) → (col, row) 1-based
/// Инвертировано по вертикали (N <-> S, NW <-> SW, NE <-> SE)
pub const BLOB_REMAP: [(u8, u8); 47] = remap! {
     0 => ( 1, 4),
     1 => ( 4, 4),
     2 => ( 1, 3),
     3 => ( 4, 3),
     4 => (12, 4),
     5 => ( 2, 4),
     6 => ( 3, 4),
     7 => ( 2, 3),
     8 => ( 3, 3),
     9 => ( 7, 4),
    10 => ( 9, 4),
    11 => ( 6, 4),
    12 => (10, 4),
    13 => ( 1, 1),
    14 => ( 4, 1),
    15 => ( 1, 2),
    16 => ( 4, 2),
    17 => ( 8, 3),
    18 => ( 2, 1),
    19 => ( 3, 1),
    20 => ( 2, 2),
    21 => ( 3, 2),
    22 => ( 5, 1),
    23 => ( 5, 3),
    24 => ( 8, 1),
    25 => (11, 4),
    26 => ( 9, 1),
    27 => ( 6, 1),
    28 => ( 5, 2),
    29 => ( 8, 4),
    30 => (11, 3),
    31 => ( 9, 2),
    32 => ( 9, 3),
    33 => ( 6, 3),
    34 => (12, 1),
    35 => ( 8, 2),
    36 => (12, 3),
    37 => ( 7, 1),
    38 => ( 5, 4),
    39 => (12, 2),
    40 => (10, 2),
    41 => ( 7, 3),
    42 => (11, 1),
    43 => (10, 1),
    44 => ( 7, 2),
    45 => ( 6, 2),
    46 => (10, 3),
};

#[inline]
#[allow(dead_code)]
pub const fn get_blob_tile_index(mask: u8) -> usize {
    let algo_idx = BLOB_MAP[mask as usize] as usize;
    let (col, row) = BLOB_REMAP[algo_idx];
    ((row - 1) as usize) * ATLAS_COLS + ((col - 1) as usize)
}

#[allow(dead_code)]
pub fn debug_print_blob_tiles() {
    let mut seen = [false; 47];

    for i in 0..256usize {
        let algo_idx = BLOB_MAP[i] as usize;
        if seen[algo_idx] { continue; }
        seen[algo_idx] = true;

        let (col, row) = BLOB_REMAP[algo_idx];

        let bits: [bool; 8] = [
            (i & 0b10000000) != 0,
            (i & 0b01000000) != 0,
            (i & 0b00100000) != 0,
            (i & 0b00010000) != 0,
            (i & 0b00001000) != 0,
            (i & 0b00000100) != 0,
            (i & 0b00000010) != 0,
            (i & 0b00000001) != 0,
        ];

        let top    = format!("{}{}{}",
            if bits[0] {"#"} else {"."},
            if bits[1] {"#"} else {"."},
            if bits[2] {"#"} else {"."});
        let middle = format!("{}X{}",
            if bits[7] {"#"} else {"."},
            if bits[3] {"#"} else {"."});
        let bottom = format!("{}{}{}",
            if bits[6] {"#"} else {"."},
            if bits[5] {"#"} else {"."},
            if bits[4] {"#"} else {"."});

        println!("id: {:2} pos: ({:2}, {:2})", algo_idx, col, row);
        println!("  |{}|", top);
        println!("  |{}|", middle);
        println!("  |{}|", bottom);
        println!();
    }
}