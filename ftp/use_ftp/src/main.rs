use std::str;
use std::io::Cursor;
use ftp::FtpStream;

fn main() {
    let mut ftp_stream = FtpStream::connect("127.0.0.1:21").unwrap();
    let _ = ftp_stream.login("tt", "1").unwrap();

    println!("Current directory: {}", ftp_stream.pwd().unwrap());
    let remote_file = ftp_stream.simple_retr("./hello").unwrap();
    println!("Contents: \n{}\n", str::from_utf8(&remote_file.into_inner()).unwrap());

    let mut reader = Cursor::new("Hello from client".as_bytes());
    let _ = ftp_stream.put("upload", &mut reader);

    let _ = ftp_stream.quit();
}
