//! Contains utlis for constructing billboard-style pixel art.
//!
//! The most common use case for this is to turn text into pixel art that can
//! scroll across the screen like on a billboard.
//!
//! The [Billboard] type alias specifies how such strips of pixel art are
//! reperesented for the purpose of this module. You can construct both
//! [horizontal] and [vertical] billboards.

/// A "billboard" is represented as a slice of arrays, where each array has
/// length 8, because 8 is the "width" of the strip of pixel art. For horizontal
/// billboards like text, such an array represents a column of pixels. For
/// vertical billboards, it's a row.
///
pub type Billboard = &'static [[bool; 8]];

/// Construct a horizontal strip of pixel art.
///
/// The strip must be 8 pixels high. After construction, you can draw the
/// billboard with [LedMatrix::draw_horizontal_billboard_section].
///
/// # Examples
///
/// ```
/// static ZIG_ZAG: Billboard = &horizontal([
///     *b"#             #             #       ",
///     *b" #           # #           # #      ",
///     *b"  #         #   #         #   #     ",
///     *b"   #       #     #       #     #    ",
///     *b"    #     #       #     #       #   ",
///     *b"     #   #         #   #         #  ",
///     *b"      # #           # #           # ",
///     *b"       #             #             #",
/// ]);
/// ```
///
pub const fn horizontal<const L: usize, const W: usize>(billboard: [[u8; L]; W]) -> [[bool; W]; L] {
    transpose(vertical(billboard))
}

/// Construct a vertical strip of pixel art.
///
/// The strip must be 8 pixels wide. After construction, you can draw the
/// billboard with [LedMatrix::draw_vertical_billboard_section].
///
/// # Examples
///
/// ```
/// static ZIG_ZAG: Billboard = &vertical([
///     *b"#       ",
///     *b" #      ",
///     *b"  #     ",
///     *b"   #    ",
///     *b"    #   ",
///     *b"     #  ",
///     *b"      # ",
///     *b"       #",
///     *b"      # ",
///     *b"     #  ",
///     *b"    #   ",
///     *b"   #    ",
///     *b"  #     ",
///     *b" #      ",
///     *b"#       ",
///     *b" #      ",
///     *b"  #     ",
///     *b"   #    ",
///     *b"    #   ",
///     *b"     #  ",
///     *b"      # ",
///     *b"       #",
/// ]);
/// ```
///
pub const fn vertical<const L: usize, const W: usize>(billboard: [[u8; W]; L]) -> [[bool; W]; L] {
    let mut res = [[false; W]; L];
    let mut i = 0;
    while i < L {
        let mut j = 0;
        while j < W {
            res[i][j] = match billboard[i][j] {
                b'#' => true,
                b' ' => false,
                _ => panic!("invalid pixel art byte"),
            };
            j += 1;
        }
        i += 1;
    }
    res
}

/// Compile-time transposition of pixel art
const fn transpose<const N: usize, const M: usize>(data: [[bool; M]; N]) -> [[bool; N]; M] {
    let mut res = [[false; N]; M];
    let mut i = 0;
    while i < M {
        let mut j = 0;

        while j < N {
            res[i][j] = data[j][i];

            j += 1;
        }
        i += 1;
    }
    res
}

/// Compile-time concatenation of pixel art
const fn concat<const H: usize, const A: usize, const B: usize, const C: usize>(
    a: [[bool; H]; A],
    b: [[bool; H]; B],
) -> [[bool; H]; C] {
    if A + B != C {
        panic!();
    }
    let mut res = [[false; H]; C];

    let mut i = 0;
    while i < A {
        res[i] = a[i];
        i += 1;
    }
    while i < C {
        res[i] = b[i];
        i += 1;
    }
    res
}
