
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

fn digit(num: u32, idx: u32) -> u8 {
    // ((num % (10**(idx+1))) - (num % (10**idx)))/(10**idx) 
    (((num % (10_u32.pow(idx+1))) - (num % (10_u32.pow(idx))))/(10_u32.pow(idx))) as u8
}

fn numlen(num: u32) -> u8 {
    let len: u8 = 0;
    while num != 0 {
        num = num / 10;
        len += 1;
    }
    len
}

pub fn number_to_vec(num: u32) -> Vec<u8> {
    let out: Vec<u8> = Vec::new();

    let len = numlen(num);

    for idx in 0..len {
        out.push(digit(num, idx as u32)); 
    }

    out
}
