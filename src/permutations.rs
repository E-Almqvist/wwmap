use permutator::Permutation;

pub fn get_ipv4_permutations(start_pos: [u8; 4]) -> Vec<Vec<u8>> {
    let perms: Vec<Vec<u8>> = Vec::new();

    // (2^8)^4 = 2^32 => 32bit
    let range: Box<[u8; u32::max_value() as usize]> = (0..u8::max_value()).collect::<Box<[u8]>>().try_into().expect("nope");


    perms
}
