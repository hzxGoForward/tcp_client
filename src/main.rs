/*
by hzx 2020-08-01
一个简易的echo tcp client 实现
*/

use std::io::{self, prelude::*, BufReader, Write};
use std::net::{Shutdown, TcpStream};
use std::str;
use std::thread;
use std::time;

fn main() -> std::io::Result<()> {
    // 创建TcpStream并尝试与服务器建立连接
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    // 设置60秒读取数据超时时间
    stream
        .set_read_timeout(Some(time::Duration::from_secs(60 as u64)))
        .expect("set_read_timeout call failed");

    // 从标准输入中持续读取数据
    loop {
        // 新建字符串input
        let mut input = String::new();
        //
        // 从标准输入中读取数据
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");
        // 如果input数据为exit，则退出程序
        if input.trim() == String::from("exit") {
            break;
        }

        // 向服务器发送数据
        stream
            .write(input.as_bytes())
            .expect("Failed to write to stream");
        // 使用BufReader读取stream中的数据
        let mut reader = BufReader::new(&stream);
        // buffer存储全部数据
        let mut buffer: Vec<u8> = Vec::new();
        // 尝试读取数据
        let res = reader.read_until(b'\n', &mut buffer);
        // 模式匹配数据
        match res {
            // 读取数据成功
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    // 输出数据
                    println!(
                        "echo: {}",
                        str::from_utf8(&buffer).expect("Could not write buffer as string")
                    );
                }
            }
            // 读取数据失败
            Err(e) => {
                // 打印错误
                eprintln!("occur error {:?}", e);
                // 退出循环
                break;
            }
        }
        // 睡眠100ms
        thread::sleep(time::Duration::from_millis(100 as u64));
    }
    // 输出退出信息
    println!("we are going to shutdown the connection");
    // 关闭连接
    stream.shutdown(Shutdown::Both).expect("shutdown error");
    Ok(())
}
