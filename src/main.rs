fn main() {
    use std::io::Read;
    use std::io::Write;
    use std::io::BufRead;

    let args: Vec<_> = std::env::args().collect();
    let mut file = std::fs::File::open(&args[2]).unwrap();
    let contents = {
        let mut tr = String::new();
        file.read_to_string(&mut tr).unwrap();
        tr
    };
    let rows: Vec<Vec<_>> = contents.split("\n\n").map(|row|row.lines().collect()).collect();

    let mut sock = std::net::TcpStream::connect(&args[1]).unwrap();
    sock.write(format!("{}{}", rows.iter().map(|row|format!("?{}", row.len())).collect::<Vec<_>>().join(""), "\n").as_bytes()).unwrap();
    sock.flush().unwrap();

    let mut br = std::io::BufReader::new(std::io::stdin());

    let mut line = String::new();

    for j in 0..rows.len() {
        let lines = &rows[j];
        for i in 0..lines.len() {
            br.read_line(&mut line).unwrap();
            println!("next line: {}", lines[i]);
            println!("{}{}", j, i);
            sock.write(format!("!{}{}{}\n", j, i, lines[i]).as_bytes()).unwrap();
            sock.flush().unwrap();
        }
    }
}
