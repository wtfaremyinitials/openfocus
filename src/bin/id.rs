fn main() {
    use openfocus::util::generate_id;
    for i in 0..100 {
        println!("{}", generate_id());
    }
}
