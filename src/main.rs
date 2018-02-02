fn main() {
    use std::io::BufRead;
    use std::io::Write;

    let args: Vec<_> = std::env::args().collect();
    let file = std::fs::File::open(&args[2]).unwrap();
    let reader = std::io::BufReader::new(&file);
    let lines: Vec<_> = reader.lines().map(|x|x.unwrap()).collect();

    let mut sock = std::net::TcpStream::connect(&args[1]).unwrap();
    sock.write(format!("?{}\n", lines.len()).as_bytes()).unwrap();
    sock.flush().unwrap();

    let mut br = std::io::BufReader::new(std::io::stdin());

    let mut line = String::new();

    for i in 0..lines.len() {
        br.read_line(&mut line);
        println!("next line: {}", lines[i]);
        sock.write(format!("{}{}\n", i, lines[i]).as_bytes()).unwrap();
        sock.flush().unwrap();
    }
}
