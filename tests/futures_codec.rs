// Test using the AsyncRead/AsyncWrite from futures 0.3
//
// ✔ frame with futures-codec
// ✔ send/receive half a frame
//


use
{
	ws_stream     :: { *                                                                              } ,
	futures       :: { StreamExt, SinkExt, channel::oneshot, executor::LocalPool, task::LocalSpawnExt } ,
	futures_codec :: { LinesCodec, Framed                                                             } ,
	// log           :: { * } ,
};


#[ test ]
//
fn frame03()
{
	// flexi_logger::Logger::with_str( "futures_codec=trace, ws_stream=trace, tokio=warn" ).start().expect( "flexi_logger");

	let mut pool     = LocalPool::new();
	let mut spawner  = pool.spawner();


	let server = async
	{
		let mut connections = TungWebSocket::listen( "127.0.0.1:3012" ).take(1);
		let     socket      = connections.next().await.expect( "1 connection" ).expect( "1 connection" ).await.expect( "WS handshake" );

		let server = WsStream::new( socket );

		let mut framed = Framed::new( server, LinesCodec {} );

		framed.send( "A line\n"       .to_string() ).await.expect( "Send a line" );
		framed.send( "A second line\n".to_string() ).await.expect( "Send a line" );
		framed.close().await.expect( "close frame" );
	};


	let client = async
	{
		let     socket = TungWebSocket::connect( "127.0.0.1:3012" ).await.expect( "connect to websocket" );
		let     client = WsStream::new( socket );
		let mut framed = Framed::new( client, LinesCodec {} );


		let res = framed.next().await.expect( "Receive some" ).expect( "Receive a line" );
		assert_eq!( "A line\n".to_string(), res );


		let res = framed.next().await.expect( "Receive some" ).expect( "Receive a second line" );
		assert_eq!( "A second line\n".to_string(), res );


		let res = framed.next().await;
		dbg!( &res );
		assert!( res.is_none() );
	};

	spawner.spawn_local( server ).expect( "spawn server" );
	spawner.spawn_local( client ).expect( "spawn client" );

	pool.run();
}


// Receive half a frame
//
#[ test ]
//
fn partial()
{
	// flexi_logger::Logger::with_str( "events=trace, wasm_websocket_stream=trace, tokio=warn" ).start().expect( "flexi_logger");

	let mut pool     = LocalPool::new();
	let mut spawner  = pool.spawner();

	let (tx, rx) = oneshot::channel();

	let server = async move
	{
		let mut connections = TungWebSocket::listen( "127.0.0.1:3013" ).take(1);
		let     socket      = connections.next().await.expect( "1 connection" ).expect( "1 connection" ).await.expect( "WS handshake" );
		let     server      = WsStream::new( socket );


		let mut framed = Framed::new( server, LinesCodec {} );

		framed.send( "A ".to_string() ).await.expect( "Send a line" );

		// Make sure the client tries to read on a partial line first.
		//
		rx.await.expect( "read channel" );

		framed.send( "line\n"         .to_string() ).await.expect( "Send a line" );
		framed.send( "A second line\n".to_string() ).await.expect( "Send a line" );

		framed.close().await.expect( "close connection" );
	};

	let client = async move
	{
		let     socket = TungWebSocket::connect( "127.0.0.1:3013" ).await.expect( "connect to websocket" );
		let     client = WsStream::new( socket );
		let mut framed = Framed::new( client, LinesCodec {} );

		// This will not return pending, so we will call framed.next() before the server task will send
		// the rest of the line.
		//
		tx.send(()).expect( "trigger channel" );
		let res = framed.next().await.expect( "Receive some" ).expect( "Receive a line" );
		assert_eq!( "A line\n".to_string(), dbg!( res ) );


		let res = framed.next().await.expect( "Receive some" ).expect( "Receive a second line" );
		assert_eq!( "A second line\n".to_string(), dbg!( res ) );


		let res = framed.next().await;
		assert!( dbg!( res ).is_none() );
	};

	spawner.spawn_local( server ).expect( "spawn server" );
	spawner.spawn_local( client ).expect( "spawn client" );

	pool.run();
}
