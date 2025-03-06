fn main() {
    let mut num = 0;
    for i in 1..100 {
        if i % 3 == 0 && i % 5 == 0 {
            num += 1;
            println!("{}", i);
        }
    }
}
