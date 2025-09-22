# Rust UDP 服务器示例代码

这个目录包含了完整的 Rust UDP 服务器实现示例。

## 文件说明

- `server.rs` - UDP 服务器主程序
- `client.rs` - 用于测试的客户端程序  
- `Cargo.toml` - Rust 项目配置文件

## 运行步骤

1. 创建新的 Rust 项目：
   ```bash
   cargo new udp_server_tutorial
   cd udp_server_tutorial
   ```

2. 复制代码文件到项目目录：
   - 将 `server.rs` 内容复制到 `src/main.rs`
   - 将 `client.rs` 内容复制到 `src/client.rs` 
   - 将 `Cargo.toml` 内容覆盖项目根目录的 `Cargo.toml`

3. 运行服务器（在一个终端中）：
   ```bash
   cargo run --bin server
   ```

4. 运行客户端（在另一个终端中）：
   ```bash
   cargo run --bin client
   ```

## 预期输出

服务器输出：
```
UDP 服务器启动成功，监听地址: 127.0.0.1:8080
收到来自 127.0.0.1:xxxxx 的消息: Hello from client!
已向 127.0.0.1:xxxxx 发送确认消息，字节数: 16
```

客户端输出：
```
消息已发送: Hello from client!
服务器回复: Message received
```