pub fn pr(bytes: &[u8]) {
    for i in 0..=bytes.len() / 16 {
        for j in 0..16 {
            let pos = j + i * 16;
            if pos < bytes.len() {
                print!("{:02X}", bytes[pos]);
            } else {
                print!("  ");
            }
            if j % 4 == 3 {
                print!(" ");
            }
        }
        for j in 0..16 {
            let pos = j + i * 16;
            if pos < bytes.len() {
                let b = bytes[pos];
                if b.is_ascii_punctuation() || b.is_ascii_alphanumeric() {
                    let c = char::from(b);
                    print!("{}", c);
                } else {
                    print!(".");
                }
            }
        }
        println!();
    }
}
