//#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4]; // Get a slice from index 1 to index 3 (inclusive)

    assert_eq!([2, 3, 4], nice_slice)
}
