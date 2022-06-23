use embedded_nal_async::{IpAddr, Ipv4Addr, SocketAddr};
use embedded_nal_async_std::Stack;

const PORT: u16 = 9876; //

#[async_std::test]
async fn tcp_ping_pong() {
    use embedded_nal_async::{TcpClientStack, TcpFullStack};
    let mut buf = [0u8; 256];

    // Create sockets
    let mut stack = Stack::default();
    let mut client = stack.socket().await.unwrap();
    let mut server = stack.socket().await.unwrap();

    // Bind server
    stack.bind(&mut server, PORT).await.unwrap();

    // Connect client
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), PORT);
    stack.connect(&mut client, addr).await.unwrap();

    // Accept the connection
    let (mut server, _addr) = stack.accept(&mut server).await.unwrap();

    // Check if connection is ok
    assert!(stack.is_connected(&mut client).await.is_ok());

    // Send client -> server
    assert_eq!(stack.send(&mut client, b"PING").await.unwrap(), 4);
    assert_eq!(stack.receive(&mut server, &mut buf).await.unwrap(), 4);
    assert_eq!(&buf[..4], b"PING");

    // Send server -> client
    assert_eq!(stack.send(&mut server, b"PONG").await.unwrap(), 4);
    assert_eq!(stack.receive(&mut client, &mut buf).await.unwrap(), 4);
    assert_eq!(&buf[..4], b"PONG");

    // Close
    stack.close(client).await.unwrap();
    stack.close(server).await.unwrap();
}

#[async_std::test]
async fn udp_ping_pong() {
    use embedded_nal_async::{UdpClientStack, UdpFullStack};
    let mut buf = [0u8; 256];

    // Create sockets
    let mut stack = Stack::default();
    let mut client = stack.socket().await.unwrap();
    let mut server = stack.socket().await.unwrap();

    // Bind server
    stack.bind(&mut server, PORT).await.unwrap();

    // Connect client
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), PORT);
    stack.connect(&mut client, addr).await.unwrap();

    // Send client -> server
    stack.send(&mut client, b"PING").await.unwrap();
    let (n, remote) = stack.receive(&mut server, &mut buf).await.unwrap();
    assert_eq!(n, 4);
    assert_eq!(&buf[..4], b"PING");

    // Send server -> client
    stack.send_to(&mut server, remote, b"PONG").await.unwrap();
    let (n, _remote) = stack.receive(&mut client, &mut buf).await.unwrap();
    assert_eq!(n, 4);
    assert_eq!(&buf[..4], b"PONG");

    // Close
    stack.close(client).await.unwrap();
    stack.close(server).await.unwrap();
}
