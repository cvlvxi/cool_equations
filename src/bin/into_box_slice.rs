// fn some_sized_fn<T: Sized>(a: T) {
// }
fn main() {



    let some_vec = vec![1,2,3,4,5];
    let mut box_slice: Box<[i32]> = some_vec.into_boxed_slice();


    box_slice.len();
    box_slice[0] = 100;
    println!("{:?}", box_slice);

    let two_d_vec = vec![vec![1,2,3,4]];
    let box_slice_2 = two_d_vec.into_boxed_slice();

    println!("{:?}", box_slice_2.len());

    // some_sized_fn(box_slice);
}