fn main() {
    let noodles = "noodles".to_string();
    let _oodles = &noodles[1..];

    let _poodles = "ಠ_ಠ";

    assert_eq!("ಠ_ಠ".len(), 7);
    assert_eq!("ಠ_ಠ".chars().count(), 3);

    let mut s = "hello";
    s[0] = 'c';
    // s.push('\n');
}
