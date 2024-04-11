pub fn slice_array() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3]; // this must be exclusive of bounds 🤔

    assert_eq!(slice, &[2, 3]); // this asserts that the value of slice is equal to a fake array of 2 and 3. kinda like go with
                                // pointers to bogus structs:
                                // &{
                                // ... struct contents
                                // }
    println!("{:#?}", slice);

    assert_eq!(slice, &[2, 3]); // this will fail with a panic
}
