use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;

fn handle_client(mut stream: TcpStream) -> Result<(), Error>{

    //新建一个长度为512的数字，初始内容为0
    let mut buf = [0; 512];
    loop {
        //从流中读取一行数据，数据长度赋值给bytes_read，数据内容赋值给&mut buf
        let bytes_read = stream.read(&mut buf)?;

        //如果数据长度为0则返回成功
        if bytes_read == 0 {
            return Ok(());
        }

        //将数组的数据重新写入流
        stream.write(&buf[..bytes_read])?;
        //休眠1秒
        thread::sleep(time::Duration::from_secs(1 as u64));
    }
    //执行完则返回成功
    Ok(())
}

//
fn main() -> std::io::Result<()> {
    //监听端口
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();

    for stream in listener.incoming() {

        //读取流若出现一场则抛出异常信息failed
        let stream = stream.expect("failed!");

        //创建一个线程，通过move操作符将stream数据移动到子线程内
        let handle = thread::spawn(move || {
            //在子线程中调用handle_client方法，并对结果做一个模式匹配
            match handle_client(stream) {
                Ok(v) => v,
                Err(e) => panic!(e)
            }
        });
        将每个子线程都放到容器里
        thread_vec.push(handle);//
    }
    
    //遍历容器，调用join方法，使主线程等待所有子线程结束后才能结束
    for handle in thread_vec {
        handle.join().unwrap();
    }

    Ok(())
}
