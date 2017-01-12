// Copyright 2015 M. Rizky Luthfianto.
// Licensed under the MIT license (http://opensource.org/licenses/MIT)
// This file may not be copied, modified, or distributed
// except according to those terms.

use nalgebra::DMatrix;

lazy_static! {

// taken from https://github.com/seqan/seqan/blob/master/include%2Fseqan%2Fscore%2Fscore_matrix_data.h#L614
	static ref ARRAY: [i32;729]=[
		 3,  0, -3,  0,  0, -4,  1, -3, -1, -2, -2, -3, -2, -1, -1,  1, -1, -3,  1,  1, -1,  0, -7, -4, -1, -1, -8,
		 0,  4, -6,  4,  3, -5,  0,  1, -3, -4,  0, -4, -4,  3, -1, -2,  0, -2,  0,  0, -1, -3, -6, -3,  2, -1, -8,
		-3, -6,  9, -7, -7, -6, -4, -4, -3, -5, -7, -7, -6, -5, -4, -4, -7, -4,  0, -3, -4, -3, -8, -1, -7, -4, -8,
		 0,  4, -7,  5,  3, -7,  0,  0, -3, -4, -1, -5, -4,  2, -2, -3,  1, -3,  0, -1, -2, -3, -8, -5,  3, -2, -8,
		 0,  3, -7,  3,  5, -7, -1, -1, -3, -4, -1, -4, -3,  1, -1, -2,  2, -3, -1, -2, -1, -3, -8, -5,  4, -1, -8,
		-4, -5, -6, -7, -7,  8, -5, -3,  0,  0, -7,  0, -1, -4, -3, -5, -6, -5, -3, -4, -3, -3, -1,  4, -6, -3, -8,
		 1,  0, -4,  0, -1, -5,  5, -4, -4, -5, -3, -5, -4,  0, -2, -2, -3, -4,  1, -1, -2, -2, -8, -6, -2, -2, -8,
		-3,  1, -4,  0, -1, -3, -4,  7, -4, -4, -2, -3, -4,  2, -2, -1,  3,  1, -2, -3, -2, -3, -3, -1,  1, -2, -8,
		-1, -3, -3, -3, -3,  0, -4, -4,  6,  4, -3,  1,  1, -2, -1, -3, -3, -2, -2,  0, -1,  3, -6, -2, -3, -1, -8,
		-2, -4, -5, -4, -4,  0, -5, -4,  4,  4, -4,  3,  2, -3, -2, -3, -3, -3, -3, -2, -2,  2, -5, -2, -3, -2, -8,
		-2,  0, -7, -1, -1, -7, -3, -2, -3, -4,  5, -4,  0,  1, -2, -2,  0,  2, -1, -1, -2, -4, -5, -5, -1, -2, -8,
		-3, -4, -7, -5, -4,  0, -5, -3,  1,  3, -4,  5,  3, -4, -2, -3, -2, -4, -4, -3, -2,  1, -3, -2, -3, -2, -8,
		-2, -4, -6, -4, -3, -1, -4, -4,  1,  2,  0,  3,  8, -3, -2, -3, -1, -1, -2, -1, -2,  1, -6, -4, -2, -2, -8,
		-1,  3, -5,  2,  1, -4,  0,  2, -2, -3,  1, -4, -3,  4, -1, -2,  0, -1,  1,  0, -1, -3, -4, -2,  0, -1, -8,
		-1, -1, -4, -2, -1, -3, -2, -2, -1, -2, -2, -2, -2, -1, -2, -2, -1, -2, -1, -1, -2, -1, -5, -3, -1, -2, -8,
		 1, -2, -4, -3, -2, -5, -2, -1, -3, -3, -2, -3, -3, -2, -2,  6,  0, -1,  1, -1, -2, -2, -7, -6, -1, -2, -8,
		-1,  0, -7,  1,  2, -6, -3,  3, -3, -3,  0, -2, -1,  0, -1,  0,  6,  1, -2, -2, -1, -3, -6, -5,  4, -1, -8,
		-3, -2, -4, -3, -3, -5, -4,  1, -2, -3,  2, -4, -1, -1, -2, -1,  1,  6, -1, -2, -2, -3,  1, -5, -1, -2, -8,
		 1,  0,  0,  0, -1, -3,  1, -2, -2, -3, -1, -4, -2,  1, -1,  1, -2, -1,  3,  2, -1, -2, -2, -3, -1, -1, -8,
		 1,  0, -3, -1, -2, -4, -1, -3,  0, -2, -1, -3, -1,  0, -1, -1, -2, -2,  2,  4, -1,  0, -6, -3, -2, -1, -8,
		-1, -1, -4, -2, -1, -3, -2, -2, -1, -2, -2, -2, -2, -1, -2, -2, -1, -2, -1, -1, -2, -1, -5, -3, -1, -2, -8,
		 0, -3, -3, -3, -3, -3, -2, -3,  3,  2, -4,  1,  1, -3, -1, -2, -3, -3, -2,  0, -1,  5, -8, -3, -3, -1, -8,
		-7, -6, -8, -8, -8, -1, -8, -3, -6, -5, -5, -3, -6, -4, -5, -7, -6,  1, -2, -6, -5, -8, 12, -2, -7, -5, -8,
		-4, -3, -1, -5, -5,  4, -6, -1, -2, -2, -5, -2, -4, -2, -3, -6, -5, -5, -3, -3, -3, -3, -2,  8, -5, -3, -8,
		-1,  2, -7,  3,  4, -6, -2,  1, -3, -3, -1, -3, -2,  0, -1, -1,  4, -1, -1, -2, -1, -3, -7, -5,  4, -1, -8,
		-1, -1, -4, -2, -1, -3, -2, -2, -1, -2, -2, -2, -2, -1, -2, -2, -1, -2, -1, -1, -2, -1, -5, -3, -1, -2, -8,
		-8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8,  1
	];

	static ref MAT: DMatrix<i32> = DMatrix::from_column_vector(27, 27, &*ARRAY);
}

#[inline]
fn lookup(a: u8) -> usize {
    if a == b'Y' {
        23 as usize
    } else if a == b'Z' {
        24 as usize
    } else if a == b'X' {
        25 as usize
    } else if a == b'*' {
        26 as usize
    } else {
        (a - 65) as usize
    }
}

pub fn pam120(a: u8, b: u8) -> i32 {
    let a = lookup(a);
    let b = lookup(b);

    MAT[(a, b)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pam120() {
        let score1 = pam120(b'A', b'A');
        assert_eq!(score1, 3);
        let score2 = pam120(b'*', b'*');
        assert_eq!(score2, 1);
        let score3 = pam120(b'A', b'*');
        assert_eq!(score3, -8);
        let score4 = pam120(b'*', b'*');
        assert_eq!(score4, 1);
        let score5 = pam120(b'X', b'X');
        assert_eq!(score5, -2);
        let score6 = pam120(b'X', b'Z');
        assert_eq!(score6, -1);
    }
}
