use std::net::UdpSocket;
use std::io;

fn main() -> io::Result<()> {
    // 创建客户端套接字
    let socket = UdpSocket::bind("127.0.0.1:0")?; // 使用任意可用端口
    let server_address = "127.0.0.1:8080";
    
    // 发送消息到服务器
    let message = "Hello from client!";
    socket.send_to(message.as_bytes(), server_address)?;
    println!("消息已发送: {}", message);
    
    // 接收服务器的确认消息
    let mut buffer = [0; 1024];
    let (bytes_received, _) = socket.recv_from(&mut buffer)?;
    let response = String::from_utf8_lossy(&buffer[..bytes_received]);
    
    println!("服务器回复: {}", response);
    
    Ok(())
}