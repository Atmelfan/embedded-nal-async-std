//#![cfg_attr(not(feature = "async-std"), no_std)]

use embedded_nal_async::{TcpClientStack, TcpFullStack};
use futures::{
    executor::LocalPool,
    task::{LocalSpawn, LocalSpawnExt},
};

//#[cfg(feature = "async-std")]
use embedded_nal_async_std::Stack;
//#[cfg(not(feature = "async-std"))]
//use embedded_nal_async_something::Stack;

async fn handle_connection<STACK>(
    stack: &mut STACK,
    socket: &mut STACK::TcpSocket,
) -> Result<(), STACK::Error>
where
    STACK: TcpFullStack,
{
    let mut buffer = [0u8; 256];

    loop {
        let len = stack.receive(socket, &mut buffer).await?;
        if len == 0 {
            break;
        }
        stack.send(socket, &buffer[..len]).await?;
    }

    Ok(())
}

async fn accept<STACK, S>(
    stack: &mut STACK,
    mut socket: STACK::TcpSocket,
    spawner: S,
) -> Result<(), STACK::Error>
where
    STACK: TcpFullStack + Clone + 'static,
    S: LocalSpawn,
{
    stack.bind(&mut socket, 5223).await?;

    while let Ok((mut s, addr)) = stack.accept(&mut socket).await {
        println!("Accepted from {addr}");

        // Make a clone of stack that client task may use
        // Note: Stack must be clonable and cannot lock a shared resource over an await!
        let mut local_stack = stack.clone();
        let _handle = spawner.spawn_local(async move {
            let res = handle_connection(&mut local_stack, &mut s).await;
            if let Err(err) = res {
                println!("{addr} disconnected: {err:?}");
            } else {
                println!("{addr} disconnected");
            }
        });
    }

    Ok(())
}

#[async_std::main]
async fn main() -> Result<(), async_std::io::Error> {
    //#[cfg(feature = "async-std")]
    let mut stack = Stack::default();
    //#[cfg(not(feature = "async-std"))]
    //let mut stack = Stack::some_platform_dependan_init();

    let socket = stack.socket().await?;

    let mut pool = LocalPool::new();
    let spawner = pool.spawner();

    // Start accepting clients
    let accept = accept(&mut stack, socket, spawner);
    pool.run_until(accept)
}
