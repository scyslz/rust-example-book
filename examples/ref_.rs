fn example() {
    let vec = vec![1, 2, 3];
    // let &y = &vec;
    let a = 1;
    let b = &a;
    let &c = &a;


    println!("&a {:p}", &a);
    println!("b {:p}", b);
    println!("&b {:p}", &b);
    println!("&&b {:p}", &&b);
    println!("&&&b {:p}", &&&b);
    println!("&c {:p}", &c);
    println!("&&c {:p}", &&c);
    println!("&&&c {:p}", &&&c);
    println!("&&&&c {:p}", &&&&c);

    println!("&a {}", a);
    println!("b {}", b);
    println!("&b {}", &b);
    println!("&&b {}", &&b);
    println!("&&&b {}", &&&b);
    println!("&c {}", &c);
    println!("&&c {}", &&c);
    println!("&&&c {}", &&&c);
}


fn main() {
    example();
}