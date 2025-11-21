fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}", v);
    let v2 = vec![1, 2, 3];
    println!("{:?}", v2);
    let v3 = vec![0; 10];
    println!("{:?}", v3);
    let v4 = vec![1, 2, 3];
    println!("{:?}", v4);
    let v5 = vec![1, 2, 3];
    println!("{:?}", v5);

    let vec_unmutable = [1, 2, 3];
    let first = &vec_unmutable[0];
    println!("The first element is: {}", first);
    let second = &vec_unmutable[1];
    println!("The second element is: {}", second);
    let third = &vec_unmutable[2];
    println!("The third element is: {}", third);
}
