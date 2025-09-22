use std::net::UdpSocket;
use std::io;

fn main() -> io::Result<()> {
    // 定义服务器监听的地址和端口
    let server_address = "127.0.0.1:8080";
    
    // 创建 UDP socket 并绑定到指定地址
    let socket = UdpSocket::bind(server_address)?;
    println!("UDP 服务器启动成功，监听地址: {}", server_address);
    
    // 创建缓冲区来存储接收到的数据
    let mut buffer = [0; 1024];
    
    loop {
        // 接收客户端发送的消息
        match socket.recv_from(&mut buffer) {
            Ok((bytes_received, client_address)) => {
                // 将接收到的字节转换为字符串
                let message = String::from_utf8_lossy(&buffer[..bytes_received]);
                
                // 在控制台打印收到的消息
                println!("收到来自 {} 的消息: {}", client_address, message.trim());
                
                // 准备确认消息
                let confirmation = "Message received";
                
                // 向客户端发送确认消息
                match socket.send_to(confirmation.as_bytes(), client_address) {
                    Ok(bytes_sent) => {
                        println!("已向 {} 发送确认消息，字节数: {}", client_address, bytes_sent);
                    }
                    Err(e) => {
                        eprintln!("发送确认消息失败: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("接收消息时出错: {}", e);
            }
        }
    }
}