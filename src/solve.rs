//! Keccak-f[1600], written from FIPS 202.
//!
//! No crate: the permutation is five steps over twenty-five lanes, and a
//! dependency here would be the thing measured rather than the permutation.
//! What is left for the compiler to do is unroll it and keep the state in
//! registers, which is the whole of the implementation question.
use crate::fherma::{Inputs, Outputs, Point, Tensor};

/// ι's round constants, FIPS 202 Table 1.
const RC: [u64; 24] = [
    0x0000000000000001, 0x0000000000008082, 0x800000000000808A, 0x8000000080008000,
    0x000000000000808B, 0x0000000080000001, 0x8000000080008081, 0x8000000000008009,
    0x000000000000008A, 0x0000000000000088, 0x0000000080008009, 0x000000008000000A,
    0x000000008000808B, 0x800000000000008B, 0x8000000000008089, 0x8000000000008003,
    0x8000000000008002, 0x8000000000000080, 0x000000000000800A, 0x800000008000000A,
    0x8000000080008081, 0x8000000000008080, 0x0000000080000001, 0x8000000080008008,
];

/// ρ's offsets in lane order, so `RHO[5y + x]` is the standard's `r[x][y]`.
const RHO: [u32; 25] = [
    0, 1, 62, 28, 27,
    36, 44, 6, 55, 20,
    3, 10, 43, 25, 39,
    41, 45, 15, 21, 8,
    18, 2, 61, 56, 14,
];

/// π's destination for each lane: `PI[5y + x]` is where `A[x][y]` goes.
const PI: [usize; 25] = [
    0, 10, 20, 5, 15,
    16, 1, 11, 21, 6,
    7, 17, 2, 12, 22,
    23, 8, 18, 3, 13,
    14, 24, 9, 19, 4,
];

fn permute(a: &mut [u64; 25]) {
    for &rc in RC.iter() {
        // θ
        let mut c = [0u64; 5];
        for x in 0..5 {
            c[x] = a[x] ^ a[x + 5] ^ a[x + 10] ^ a[x + 15] ^ a[x + 20];
        }
        for x in 0..5 {
            let d = c[(x + 4) % 5] ^ c[(x + 1) % 5].rotate_left(1);
            for y in 0..5 {
                a[5 * y + x] ^= d;
            }
        }
        // ρ and π in one pass
        let mut b = [0u64; 25];
        for i in 0..25 {
            b[PI[i]] = a[i].rotate_left(RHO[i]);
        }
        // χ
        for y in 0..5 {
            let r = 5 * y;
            for x in 0..5 {
                a[r + x] = b[r + x] ^ (!b[r + (x + 1) % 5] & b[r + (x + 2) % 5]);
            }
        }
        // ι
        a[0] ^= rc;
    }
}

#[derive(Default)]
pub struct State;

pub fn init(_p: &Point) -> State {
    State
}

pub fn run(_state: &mut State, inp: &Inputs) -> Outputs {
    let n = inp.state.data.len() / 25;
    let mut out = Vec::with_capacity(n * 25);
    let mut a = [0u64; 25];
    for i in 0..n {
        a.copy_from_slice(&inp.state.data[i * 25..(i + 1) * 25]);
        permute(&mut a);
        out.extend_from_slice(&a);
    }
    Outputs { out: Tensor { shape: vec![n as i64, 25], data: out } }
}

pub fn free(state: State) {
    drop(state);
}
