//#![cfg_attr(not(feature = "async-std"), no_std)]

use embedded_nal_async::TcpClientStack;

//#[cfg(feature = "async-std")]
use embedded_nal_async_std::Stack;
//#[cfg(not(feature = "async-std"))]
//use embedded_nal_async_something::Stack;

async fn send_hello_world<STACK>(
    stack: &mut STACK,
    socket: &mut STACK::TcpSocket,
) -> Result<(), STACK::Error>
where
    STACK: TcpClientStack,
{
    stack.connect(socket, ([127, 0, 0, 1], 5223).into()).await?;

    stack.send(socket, b"HELLO WORLD").await?;

    let mut buffer = [0; 256];

    let n = stack.receive(socket, &mut buffer).await?;

    println!("<< {:?}", &buffer[..n]);

    Ok(())
}

#[async_std::main]
async fn main() -> Result<(), async_std::io::Error> {
    //#[cfg(feature = "async-std")]
    let mut stack = Stack::default();
    //#[cfg(not(feature = "async-std"))]
    //let mut stack = Stack::some_platform_dependan_init();

    let mut socket = stack.socket().await?;

    send_hello_world(&mut stack, &mut socket).await?;

    Ok(())
}
