
#![allow(unused)]
fn main() {
   f1();
   f2();
   f3();
}
fn f2 (){
    use std::arch::asm;

    let mut a: u64 = 4;
    let b: u64 = 4;
    let c: u64 = 4;
    unsafe {
        asm!(
        "add {0}, {1}",
        "add {0}, {2}",
        inout(reg) a,
        in(reg) b,
        in(reg) c,
        );
    };
    println!("a {}",a);
    println!("b {}",b);
    println!("c {}",c);
}

fn f1() {
    use std::arch::asm;

    let i: u64 = 3;
    let o: u64;
    unsafe {
        asm!(
        "mov {0}, {1}",
        "add {0}, 5",
        out(reg) o,
        in(reg) i,
        );
    }
   println!("i {}",i);
   println!("o {}",o);
}


fn f3() {
    use std::arch::asm;

    let mut i: u64 = 3;
    unsafe {
        asm!(
        "add {0}, 5",
        out(reg) i,   // ret 5    in: read from reg
        // inout(reg) i, // ret 8  out: write to reg and overwrite old data
        );
    }
    println!("i {}",i);
}