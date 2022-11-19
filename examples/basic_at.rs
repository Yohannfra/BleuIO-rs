use bleuio::BleuIO;

fn main() {
    let port: String = std::env::args().nth(1).expect("USAGE: ./basic_at port");

    let mut bleuio_con = BleuIO::new(&port, 1000, false);

    bleuio_con.connect().unwrap();
    bleuio_con.start_daemon().unwrap();

    println!("Sending command 'AT'");
    let ret = bleuio_con.at().unwrap();
    println!("Read: {}", ret);
}
