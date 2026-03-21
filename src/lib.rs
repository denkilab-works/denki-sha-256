// Initial hash value: first 32 bits of the fractional parts of the square roots of the first 8 primes.
// Copied into a mutable variable that the compression function updates after each 512-bit block.
// After the final block, these 8 words are concatenated to produce the 256-bit digest.
const INITIAL_HASH: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

// Round constants: first 32 bits of the fractional parts of the cube roots of the first 64 primes.
const ROUND_CONSTANTS: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

/// Pads a message according to the SHA-256 specification.
///
/// Appends a `0x80` byte (1-bit delimiter), then zero bytes until the length
/// is 448 mod 512, and finally the original message length in bits as a 64-bit
/// big-endian value. The result is always a multiple of 64 bytes (512 bits).
fn padding(message: &[u8]) -> Vec<u8> {
    let mut padded = message.to_vec();
    padded.push(0b10000000); // 0x80
    while padded.len() % 64 != 56 {
        padded.push(0x00);
    }
    let bit_len = (message.len() as u64) * 8;
    padded.extend_from_slice(&bit_len.to_be_bytes());

    padded
}

/// σ₀ (lower-case sigma 0): rotation and shift mixing used in the message schedule
/// to expand the 16 original words into 64. Combines two right-rotations (7, 18)
/// and one right-shift (3) with XOR.
fn schedule_mix_0(w: u32) -> u32 {
    w.rotate_right(7) ^ w.rotate_right(18) ^ (w >> 3)
}

/// σ₁ (lower-case sigma 1): rotation and shift mixing used in the message schedule
/// to expand the 16 original words into 64. Combines two right-rotations (17, 19)
/// and one right-shift (10) with XOR.
fn schedule_mix_1(w: u32) -> u32 {
    w.rotate_right(17) ^ w.rotate_right(19) ^ (w >> 10)
}

/// Expands a 512-bit block (64 bytes) into 64 scheduled 32-bit words.
///
/// - Words 0–15: read directly from the block as big-endian u32 values.
/// - Words 16–63: each derived by combining four earlier words through
///   `schedule_mix_0`, `schedule_mix_1`, and wrapping addition.
fn message_schedule(block: &[u8]) -> [u32; 64] {
    let mut w = [0u32; 64];
    for t in 0..16 {
        w[t] = u32::from_be_bytes(block[t * 4..(t + 1) * 4].try_into().unwrap());
    }
    for t in 16..64 {
        w[t] = schedule_mix_1(w[t - 2])
            .wrapping_add(w[t - 7])
            .wrapping_add(schedule_mix_0(w[t - 15]))
            .wrapping_add(w[t - 16]);
    }

    w
}

/// Choose function (Ch): for each bit position, selects the bit from `f` if the
/// corresponding bit in `e` is 1, otherwise selects from `g`.
fn choose(e: u32, f: u32, g: u32) -> u32 {
    (e & f) ^ (!e & g)
}

/// Majority function (Maj): for each bit position, returns 1 if at least two
/// of the three inputs have a 1 in that position.
fn majority(a: u32, b: u32, c: u32) -> u32 {
    (a & b) ^ (a & c) ^ (b & c)
}

/// Σ₀ (upper-case sigma 0): bitwise rotation mixing applied to working variable `a`
/// during each compression round. Combines three right-rotations (2, 13, 22) with XOR.
fn rotate_mix_a(a: u32) -> u32 {
    a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22)
}

/// Σ₁ (upper-case sigma 1): bitwise rotation mixing applied to working variable `e`
/// during each compression round. Combines three right-rotations (6, 11, 25) with XOR.
fn rotate_mix_e(e: u32) -> u32 {
    e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25)
}

/// Runs 64 compression rounds on a single block.
///
/// Takes the current 8-word hash state and the 64 scheduled words.
/// Returns the updated hash state where each word is the sum (mod 2³²)
/// of the input hash word and the corresponding working variable after all rounds.
fn compress(hash: &[u32; 8], w: &[u32; 64]) -> [u32; 8] {
    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = *hash;

    for t in 0..64 {
        let t1 = h
            .wrapping_add(rotate_mix_e(e))
            .wrapping_add(choose(e, f, g))
            .wrapping_add(ROUND_CONSTANTS[t])
            .wrapping_add(w[t]);
        let t2 = rotate_mix_a(a).wrapping_add(majority(a, b, c));
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(t1);
        d = c;
        c = b;
        b = a;
        a = t1.wrapping_add(t2);
    }

    [
        hash[0].wrapping_add(a),
        hash[1].wrapping_add(b),
        hash[2].wrapping_add(c),
        hash[3].wrapping_add(d),
        hash[4].wrapping_add(e),
        hash[5].wrapping_add(f),
        hash[6].wrapping_add(g),
        hash[7].wrapping_add(h),
    ]
}

/// Computes the SHA-256 digest of a message. Returns a 32-byte array (256 bits).
///
/// ```
/// let digest = denki_sha_256::sha256(b"abc");
/// let hex: String = digest.iter().map(|b| format!("{b:02x}")).collect();
/// assert_eq!(hex, "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
/// ```
pub fn sha256(message: &[u8]) -> [u8; 32] {
    let padded = padding(message);
    let mut hash = INITIAL_HASH;

    for block in padded.chunks(64) {
        let w = message_schedule(block);
        hash = compress(&hash, &w);
    }

    let mut digest = [0u8; 32];
    for (i, &word) in hash.iter().enumerate() {
        digest[i * 4..(i + 1) * 4].copy_from_slice(&word.to_be_bytes());
    }

    digest
}

#[cfg(test)]
mod tests;
