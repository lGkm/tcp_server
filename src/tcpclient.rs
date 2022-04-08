use std::io::{self, prelude::*, BufReader, Write};
use std::net::TcpStream;
use std::str;

fn main() -> std::io::Result<()> {
    // 创建可变对象 stream 链接服务器
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    for _ in 0..10 {
        //创建一个String对象
        let mut input = String::new();
        //通过输入读取一行存到 input 中，失败会报错 Failed to read from stdin
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");
        // input 转成bytes 写入到 stream 
        stream
            .write(input.as_bytes())
            .expect("Failed to write to stream");
        //通过 stream 创建 BufReader
        let mut reader = BufReader::new(&stream);
        // 创建一个 vector 变量 类型为字节
        let mut buffer: Vec<u8> = Vec::new();
        // reader 中度数据到换行为止，存入 buffer
        reader
            .read_until(b'\n', &mut buffer)
            .expect("Could not read into buffer");
        // 将buffer 打印
        println!("{}", 
            str::from_utf8(&buffer).expect("Could not write buffer as string"));
        println!("");
    }
    Ok(())
}