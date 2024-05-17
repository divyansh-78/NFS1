#[test]
fn main() {
    let mut x = 100;
    let y = &mut x;
    let z = &mut x;
    let _ = *z; // Moved the declaration of `z` before the modification of `y` and `z`.
    *y += 100;
    *z += 1000;
    assert_eq!(x, 1200);
}
