fn main() {
    print!("a is {0} , a is {0}", add(2, 3));
    test_1();
}

/// adds one to the number given
///
/// # Examples
///
/// ```
/// let x = add(1, 2)
///
/// ```

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn test_1(){
    println!("test 1");
}
