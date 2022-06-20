use embedded_nal_async::{TcpClientStack, TcpFullStack};
use embedded_nal_async_std::Stack;

async fn do_something<STACK, SOCKET, ERR>(stack: &mut STACK, mut socket: SOCKET) -> Result<(), ERR>
where
    STACK: TcpFullStack<TcpSocket = SOCKET, Error = ERR>,
{
    stack.bind(&mut socket, 5223).await?;

    while let Ok((mut s, addr)) = stack.accept(&mut socket).await {
        println!("Accepted from {addr:?}");

        stack.send(&mut s, b"").await?;
    }

    Ok(())
}

#[async_std::main]
async fn main() -> Result<(), async_std::io::Error> {
    let mut stack = Stack;

    let socket = stack.socket().await?;

    do_something(&mut stack, socket).await?;

    Ok(())
}
