use embedded_nal_async::TcpClientStack;
use embedded_nal_async_std::Stack;

async fn do_something<STACK, SOCKET, ERR>(mut stack: STACK, mut socket: SOCKET) -> Result<(), ERR>
where
    STACK: TcpClientStack<TcpSocket = SOCKET, Error = ERR>,
{
    stack
        .connect(&mut socket, ([127, 0, 0, 1], 8080).into())
        .await?;

    stack.send(&mut socket, b"HELLO WORLD").await?;

    Ok(())
}

#[async_std::main]
async fn main() -> Result<(), async_std::io::Error> {
    let mut stack = Stack;

    let socket = stack.socket().await?;

    do_something(&mut stack, socket).await?;

    Ok(())
}
