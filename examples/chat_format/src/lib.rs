pub mod futures_serde_cbor;


use
{
	serde :: { Serialize, Deserialize } ,
};


/// Wire format for communication between the server and clients
//
#[ derive( Debug, Clone, PartialEq, Eq, Serialize, Deserialize ) ]
//
pub enum ClientMsg
{
	ChatMsg(String),
	SetNick(String),
}


/// Wire format for communication between the server and clients
/// The time is in secods since epoch UTC
//
#[ derive( Debug, Clone, PartialEq, Eq, Serialize, Deserialize ) ]
//
pub enum ServerMsg
{
	ServerMsg   { time: i64, txt: String                             } ,
	ChatMsg     { time: i64, nick : String, sid: usize, txt: String  } ,
	UserJoined  { time: i64, nick : String, sid: usize               } ,
	UserLeft    { time: i64, nick : String, sid: usize               } ,
	NickChanged { time: i64, old  : String, sid: usize, new: String  } ,
	Welcome     { time: i64, users: Vec<(usize,String)>, txt: String } ,
}

