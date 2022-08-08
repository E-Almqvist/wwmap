
/*
uint ulong_len(ulong n) { // get the digit length of a number
    int len = 0;
    while (n != 0) {
        n = n / 10;
        ++len;
    }
    return len;
}
*/

fn digit(num: u64, idx: u32) -> u8 {
    // ((num % (10**(idx+1))) - (num % (10**idx)))/(10**idx) 
    (((num % (10_u64.pow(idx+1))) - (num % (10_u64.pow(idx))))/(10_u64.pow(idx))) as u8
}

fn numlen(mut num: u64) -> u8 {
    let mut len: u8 = 0;

    if num == 0 {
        return 1;
    }

    while num != 0 {
        num = num / 10;
        len += 1;
    }
    len
}

pub fn number_to_vec(num: u64) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();

    let len = numlen(num);
    println!("len={len}");

    for idx in 0..len {
        let digit = digit(num, idx as u32);
        println!("\t{:?}", digit);
        out.push(digit); 
    }

    println!("{:?}", out);

    out
}
