use std::io::{self, prelude::*, BufReader, Write};
use std::net::TcpStream;
use std::str;
//客户端
fn main() -> std::io::Result<()> {
    //创建tcp连接
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    //循环体
    loop {
        //创建可变字符串
        let mut input = String::new();
        //读取命令行中输入的值 并赋值给input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin")
        //将input转换成字节码并写入流
        stream
            .write(input.as_bytes())
            .expect("Failed to write to stream");

        //创建流的读取句柄
        let mut reader = BufReader::new(&stream);

        let mut buffer: Vec<u8> = Vec::new();

        //读取一行流中的内容，以回车符作为结束
        reader
            .read_until(b'\n', &mut buffer)
            .expect("Could not read into buffer");

        //打印读取流的数据
        println!("{}",
                 str::from_utf8(&buffer).expect("Could not write buffer as string"));

    }
    Ok(())
}
