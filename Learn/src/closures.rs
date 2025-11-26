pub fn closures() {
    let add_v1 = |x: i32, y: i32| -> i32 { x + y };
    println!("Add v1: {}", add_v1(1, 2));

    let add_v2 = |x: i32, y: i32| x + y;
    println!("Add v2: {}", add_v2(2, 3));

    let add_v3 = |x, y| x + y;
    println!("Add v3: {}", add_v3(3, 4));

    fn call_with_one<F>(closure: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        closure(1)
    }

    let aws = call_with_one(|x| x + 2);
    println!("Awser: {}", aws);
}
