use bleuio::BleuIO;

fn main() {
    let port: String = std::env::args().nth(1).expect("no port given");
    let mut b = BleuIO::new(&port, 1000, false);
    b.connect().unwrap();
}
