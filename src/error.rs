use crate::{ import::* };


/// The error type for errors happening in `async_runtime`.
///
/// Use [`WsErr::kind()`] to know which kind of error happened. [WsErrKind] implements [Eq],
/// so you can do the following if all you want to know is the kind of error:
///
/// ```ignore
/// use async_runtime::*;
///
/// rt::init( RtConfig::Local ).expect( "Set default executor" );
///
/// match rt::init( RtConfig::Pool )
/// {
///    Err(e) =>
///    {
///       if let WsErrKind::DoubleExecutorInit = e.kind()
///       {
///          println!( "{}", e );
///       }
///
///       // This also works:
///       //
///       match e.kind()
///       {
///          WsErrKind::DoubleExecutorInit => println!( "{}", e ),
///          _ => {},
///       }
///    },
///
///    Ok(_) => {}
/// }
/// ```
//
#[ derive( Debug ) ]
//
pub struct WsErr
{
	inner: FailContext<WsErrKind>,
}



/// The different kind of errors that can happen when you use the `async_runtime` API.
//
#[ derive( Clone, PartialEq, Eq, Debug, Fail ) ]
//
pub enum WsErrKind
{
	/// This is an error from tokio-tungstenite.
	//
	#[ fail( display = "The WebSocket handshake failed" ) ]
	//
	WsHandshake,

	/// This is an error from tokio-tungstenite.
	//
	#[ fail( display = "A tcp connection error happened" ) ]
	//
	TcpConnection,

	/// Invalid input to `WsReadyState::try_from( u16 )`
	///
	#[ fail( display = "Invalid input to conversion to WsReadyState: {}", _0 ) ]
	//
	InvalidReadyState( u16 ),
}



impl Fail for WsErr
{
	fn cause( &self ) -> Option< &dyn Fail >
	{
		self.inner.cause()
	}

	fn backtrace( &self ) -> Option< &Backtrace >
	{
		self.inner.backtrace()
	}
}



impl fmt::Display for WsErr
{
	fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result
	{
		fmt::Display::fmt( &self.inner, f )
	}
}


impl WsErr
{
	/// Allows matching on the error kind
	//
	pub fn kind( &self ) -> &WsErrKind
	{
		self.inner.get_context()
	}
}

impl From<WsErrKind> for WsErr
{
	fn from( kind: WsErrKind ) -> WsErr
	{
		WsErr { inner: FailContext::new( kind ) }
	}
}

impl From< FailContext<WsErrKind> > for WsErr
{
	fn from( inner: FailContext<WsErrKind> ) -> WsErr
	{
		WsErr { inner }
	}
}


// TODO: this no longer compiles. It compiles fine in thespis, but not in this crate even though this
// file is largely copy/paste. The problem is that there is a blanket impl for Fail in failure for every
// E: std::error::Error + 'static + Send + Sync
//
// impl std::error::Error for WsErr {}


