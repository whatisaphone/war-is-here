use std::char;

pub fn sans_null_terminator(s: &mut [u16]) -> &mut [u16] {
    s.split_mut(|&ch| ch == 0).next().unwrap()
}

pub fn make_ascii_lowercase(s: &mut [u16]) {
    for ch in s.iter_mut() {
        *ch = char::from_u32(u32::from(*ch)).unwrap().to_ascii_lowercase() as u16;
    }
}
