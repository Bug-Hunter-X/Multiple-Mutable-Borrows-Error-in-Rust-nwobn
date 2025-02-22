fn main() {
    let mut x = 5;
    { //Added a new scope
        let y = &mut x;
        *y += 1;
    }
    { //Added a new scope
        let z = &mut x;
        *z += 1;
    }
    println!("{}", x);
}