fn main() {
    use openfocus::util::generate_id;
    for _ in 0..100 {
        println!("{}", generate_id());
    }
}
