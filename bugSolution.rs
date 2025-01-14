fn main() {
    let mut v = vec![1, 2, 3];
    //Instead of using raw pointer, directly modify the vector element.
    v[0] = 10;
    println!("{:?}", v);
}