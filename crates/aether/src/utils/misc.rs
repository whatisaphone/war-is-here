pub fn common_prefix<'a>(it: impl IntoIterator<Item = &'a str>) -> &'a str {
    let mut it = it.into_iter();
    let mut a = match it.next() {
        Some(x) => x,
        None => return "",
    };
    for x in it {
        let count = a.bytes().zip(x.bytes()).take_while(|(p, q)| p == q).count();
        a = &a[..count];
    }
    a
}
