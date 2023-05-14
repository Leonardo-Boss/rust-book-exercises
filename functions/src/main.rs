fn main() {
    let x = another_function(5);
    println!("{x}");
}

fn another_function(x: i32) -> i32{
    if x==5 {5} else {6}
}
