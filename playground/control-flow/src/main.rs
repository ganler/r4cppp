fn message(flag: bool) -> &'static str {
    if flag { // mandatory braces.
        "FLAG => TRUE"
    } else {
        "FLAG => FALSE"
    }
}

fn print_all(all: Vec<i32>) {
    for (i, v) in all.iter().enumerate() {
        println!("{}: {}", i, v)
    }
    for i in 0..all.len() {
        println!("{}: {}", i, all[i])
    }
}

fn square_all(all: &mut Vec<i32>) { // reference.
    for v in all.iter_mut() { // v -> ptrs.
        (*v) = (*v) * (*v);
    }
}

fn print_some(x: i32) {
    match x {
        0 | 1=> println!("x is zero or one"),
        10 => println!("x is ten"),
        y if y < 0 => println!("x is something else < 20: {}", y),
        _ => {} // for wildcard match.
    }
}

fn main() {
    println!("{}", message(true));
    let mut i = 5;
    while i != 0 { // or: loop => forever...
        println!("Cur val i = {}", i);
        i -= 1;
    }
    let mut range0: Vec<i32> = (0..5).collect();
    let range1 = vec![1, 2, 3, 4];
    square_all(&mut range0);
    print_all(range0);
    print_all(range1);
    print_some(i);
}