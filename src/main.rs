use std::cell::RefCell;
use std::net::{TcpListener, TcpStream, SocketAddr, IpAddr, Ipv4Addr};
use std::{io, time};
use std::io::{prelude::*, BufReader};
use std::thread;
use uuid::Uuid;
use rustirc::server::Server;
use rustirc::user::{User, IrcClient};
use rustirc::channel::{Channel, ChannelCreator};
use rustirc::client::Client;
use std::rc::Rc;
const MAX_LEN: u32 = 510;

const CR_LF: &str = "\r\n";

// const RPL_WELCOME: &str = "001";
// const RPL_YOURHOST: &str = "002";
// const RPL_CREATED: &str = "003";
// const RPL_MYINFO: &str = "004";
// const RPL_LUSERCLIENT: &str = "251";
// const RPL_LUSEROP: &str = "252";
// const RPL_LUSERUNKNOWN: &str = "253";
// const RPL_LUSERCHANNELS: &str = "254";
// const RPL_LUSERME: &str = "255";
// const RPL_AWAY: &str = "301";
// const RPL_UNAWAY: &str = "305";
// const RPL_NOWAWAY: &str = "306";
// const RPL_WHOISUSER: &str = "311";
// const RPL_WHOISSERVER: &str = "312";
// const RPL_WHOISOPERATOR: &str = "313";
// const RPL_WHOISIDLE: &str = "317";
// const RPL_ENDOFWHOIS: &str = "318";
// const RPL_WHOISCHANNELS: &str = "319";
// const RPL_WHOREPLY: &str = "352";
// const RPL_ENDOFWHO: &str = "315";
// const RPL_LIST: &str = "322";
// const RPL_LISTEND: &str = "323";
// const RPL_CHANNELMODEIS: &str = "324";
// const RPL_NOTOPIC: &str = "331";
// const RPL_TOPIC: &str = "332";
// const RPL_NAMREPLY: &str = "353";
// const RPL_ENDOFNAMES: &str = "366";
// const RPL_MOTDSTART: &str = "375";
// const RPL_MOTD: &str = "372";
// const RPL_ENDOFMOTD: &str = "376";
// const RPL_YOUREOPER: &str = "381";
// const ERR_NOSUCHNICK: &str = "401";
// const ERR_NOSUCHSERVER: &str = "402";
// const ERR_NOSUCHCHANNEL: &str = "403";
// const ERR_CANNOTSENDTOCHAN: &str = "404";
// const ERR_NORECIPIENT: &str = "411";
// const ERR_NOTEXTTOSEND: &str = "412";
// const ERR_UNKNOWNCOMMAND: &str = "421";
// const ERR_NOMOTD: &str = "422";
// const ERR_NONICKNAMEGIVEN: &str = "431";
const ERR_NICKNAMEINUSE: &str = "433";
// const ERR_USERNOTINCHANNEL: &str = "441";
// const ERR_NOTONCHANNEL: &str = "442";
// const ERR_NOTREGISTERED: &str = "451";
// const ERR_NEEDMOREPARAMS: &str = "461";
// const ERR_ALREADYREGISTRED: &str = "462";
// const ERR_PASSWDMISMATCH: &str = "464";
// const ERR_UNKNOWNMODE: &str = "472";
// const ERR_NOPRIVILEGES: &str = "481";
// const ERR_CHANOPRIVSNEEDED: &str = "482";
// const ERR_UMODEUNKNOWNFLAG: &str = "501";
// const ERR_USERSDONTMATCH: &str = "502";

pub enum Recipient {
    Server,
    User,
}

pub enum Commands {
    Connect,
    Join,
    Part,
    PrivMsg
}


#[derive(Clone)]
struct RegisteredClient {
    user_nick: String,
}
#[derive(Clone)]
struct Network {
    // Use petgraph to implement this.
    servers: Vec<Server>,
}

fn main() {

    let proto_user = User {
        nickname: Rc::new(RefCell::new("ProtoUser".to_string())),
        client: IrcClient::FooIrc ,
        full_name: "Proto User".to_string()
    };

    let user_vec = vec![proto_user.clone()];

    let channel_owner = ChannelCreator(proto_user.nickname.borrow().to_string());

    let proto_channel = Channel {
        name: "ProtoChannel".to_string(),
        users: user_vec.clone(),
        creator: channel_owner
        
    };

    let channel_vec = vec![proto_channel];

    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),7878);
    let server_nud = Uuid::new_v4();
    let mut proto_server = Server {
        channels: channel_vec,
        connected_users: user_vec,
        server_name: Rc::new(RefCell::new(String::from("ProtoServer"))),
        socket_addr: socket,
        nud: server_nud
    };

    let proto_client = Client {
        server: proto_server.clone(),
        user: proto_user.clone()
    };

    
    // Either within the function `run` itself, or prior to calling it,
    // we need to set up writing to stdout.
    proto_server.run();

    proto_client.register_client(&proto_user, &mut proto_server, false);

    let conn_user = proto_server.handle_registration(proto_client, proto_user);

    println!("Connected users: {:?}", conn_user);

}

