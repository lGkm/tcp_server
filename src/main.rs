use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;
use std::str;

fn handle_client(mut stream: TcpStream) -> Result<(), Error>{
    //定义 buf 长度为512，内容为0的数组
    let mut buf = [0; 512];
    //循环1000次处理数据之后就关闭，也可以用loop 表示一直提供服务
    for _ in 0..1000 {
        // stream 读取数据到 buf ,读取到的数据长度存入 bytes_read
        let bytes_read = stream.read(&mut buf)?;
        // bytes_read 为零 说明没有读到数据
        if bytes_read == 0 {
            return Ok(());
        }
        let ss = str::from_utf8(&buf[..bytes_read]).unwrap();
        //打印收到的内容
        println!("收到{:?}", ss.replace("\n",""));
        //向 stream 写会读到的数据
        stream.write(&buf[..bytes_read])?;
        // 延时一秒
        thread::sleep(time::Duration::from_secs(1 as u64));
    }

    Ok(())
}

fn main() -> std::io::Result<()>{
    // 定义 listener bind 中的是要监听的 ip 和端口号，？ 会在失败时 return Err ，是返回错误的语法糖，使用了？但是main 返回空会编译失败
    //let listener = TcpListener::bind("127.0.0.1:8080")?;
    // 标准错误处理方法
    let listener = TcpListener::bind("127.0.0.1:8090");
    let listener = match listener{
        Ok(listener) => listener,
        Err(error) => panic!("TcpListener::bind err: {:?}", error),
    };
    // mut 创建可变的 vec 数组来储存线程的句柄
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    // 处理链接，每一个链接是一个进程
    for stream in listener.incoming(){
        //stream 是result，可以使用 match 处理，也可以使用 expect 将会显示指定的错误信息
        // let 重新定义 stream 是 rust 的 shadow 特性，区别于 mut，实际是创建了一个新的变量
        let stream = stream.expect("failed");
        //创建一个线程处理闭包定义的方法
        //使用 move 的闭包会将外部的 stream 变量引入闭包中使用，注意被引入的变量，main将不会再有使用权
        let handle = thread::spawn(move ||{
            //unwrap_or_else 就是 match 到 Err 打印 error 信息
            handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}",error));
        });
        // 把句柄放到数组中
        thread_vec.push(handle);

    }
    for handle in thread_vec{
        // 主线程需要等所有子线程结束才能结束
        handle.join().unwrap();
    }
    Ok(())
}