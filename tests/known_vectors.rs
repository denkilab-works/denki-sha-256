use denki_sha_256::sha256;

fn digest_to_hex(digest: [u8; 32]) -> String {
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

#[test]
fn empty_string() {
    let digest = sha256(b"");
    assert_eq!(
        digest_to_hex(digest),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}

#[test]
fn abc() {
    let digest = sha256(b"abc");
    assert_eq!(
        digest_to_hex(digest),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}

#[test]
fn two_block_message() {
    let digest = sha256(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq");
    assert_eq!(
        digest_to_hex(digest),
        "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
    );
}

#[test]
fn longer_message() {
    let digest = sha256(b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu");
    assert_eq!(
        digest_to_hex(digest),
        "cf5b16a778af8380036ce59e7b0492370b249b11e8f07a51afac45037afee9d1"
    );
}

#[test]
fn single_char() {
    let digest = sha256(b"a");
    assert_eq!(
        digest_to_hex(digest),
        "ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb"
    );
}

#[test]
fn digits() {
    let digest = sha256(b"0123456789");
    assert_eq!(
        digest_to_hex(digest),
        "84d89877f0d4041efb6bf91a16f0248f2fd573e6af05c19f96bedb9f882f7882"
    );
}
