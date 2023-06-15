use core::fmt;
use std::fmt::Error;
use std::net::{TcpListener, TcpStream, SocketAddr, IpAddr, Ipv4Addr};
use std::io::{prelude::*, BufReader};
use std::rc::Rc;


const MAX_LEN: u32 = 510;

const CR_LF: &str = "\r\n";

const RPL_WELCOME: &str = "001";
const RPL_YOURHOST: &str = "002";
const RPL_CREATED: &str = "003";
const RPL_MYINFO: &str = "004";
const RPL_LUSERCLIENT: &str = "251";
const RPL_LUSEROP: &str = "252";
const RPL_LUSERUNKNOWN: &str = "253";
const RPL_LUSERCHANNELS: &str = "254";
const RPL_LUSERME: &str = "255";
const RPL_AWAY: &str = "301";
const RPL_UNAWAY: &str = "305";
const RPL_NOWAWAY: &str = "306";
const RPL_WHOISUSER: &str = "311";
const RPL_WHOISSERVER: &str = "312";
const RPL_WHOISOPERATOR: &str = "313";
const RPL_WHOISIDLE: &str = "317";
const RPL_ENDOFWHOIS: &str = "318";
const RPL_WHOISCHANNELS: &str = "319";
const RPL_WHOREPLY: &str = "352";
const RPL_ENDOFWHO: &str = "315";
const RPL_LIST: &str = "322";
const RPL_LISTEND: &str = "323";
const RPL_CHANNELMODEIS: &str = "324";
const RPL_NOTOPIC: &str = "331";
const RPL_TOPIC: &str = "332";
const RPL_NAMREPLY: &str = "353";
const RPL_ENDOFNAMES: &str = "366";
const RPL_MOTDSTART: &str = "375";
const RPL_MOTD: &str = "372";
const RPL_ENDOFMOTD: &str = "376";
const RPL_YOUREOPER: &str = "381";
const ERR_NOSUCHNICK: &str = "401";
const ERR_NOSUCHSERVER: &str = "402";
const ERR_NOSUCHCHANNEL: &str = "403";
const ERR_CANNOTSENDTOCHAN: &str = "404";
const ERR_NORECIPIENT: &str = "411";
const ERR_NOTEXTTOSEND: &str = "412";
const ERR_UNKNOWNCOMMAND: &str = "421";
const ERR_NOMOTD: &str = "422";
const ERR_NONICKNAMEGIVEN: &str = "431";
const ERR_NICKNAMEINUSE: &str = "433";
const ERR_USERNOTINCHANNEL: &str = "441";
const ERR_NOTONCHANNEL: &str = "442";
const ERR_NOTREGISTERED: &str = "451";
const ERR_NEEDMOREPARAMS: &str = "461";
const ERR_ALREADYREGISTRED: &str = "462";
const ERR_PASSWDMISMATCH: &str = "464";
const ERR_UNKNOWNMODE: &str = "472";
const ERR_NOPRIVILEGES: &str = "481";
const ERR_CHANOPRIVSNEEDED: &str = "482";
const ERR_UMODEUNKNOWNFLAG: &str = "501";
const ERR_USERSDONTMATCH: &str = "502";

pub enum Recipient {
    Server,
    User,
}

pub enum Commands {
    Join,
    Part,
    PrivMsg
}

#[derive(Clone, Debug)]
pub enum IrcClient {
    MIrc,
    FooIrc
}

#[derive(Clone)]
struct Server {
    channels: Vec<Channel>,
    pub connected_users: Vec<User>,
    server_name: String,
    socket_addr: SocketAddr
}

impl Server {
 fn new(channels: Vec<Channel>, 
    connected_users: Vec<User>, 
    registered_clients: Vec<RegisteredClient>,
    server_name: String) -> Self {
    let proto_user = User {
        nickname: "ProtoUser".to_string(),
        client: IrcClient::FooIrc ,
        full_name: "Proto User".to_string()
    };
    let user_vec = vec![proto_user.clone()];
    let channel_owner = ChannelCreator(proto_user.nickname.clone());
    let proto_channel = Channel {
        name: "ProtoChannel".to_string(),
        users: user_vec.clone(),
        creator: channel_owner
        
    };
    
    let reg_client = RegisteredClient {
        user_nick: proto_user.nickname,
    };
    let reg_vec = vec![reg_client];
    let channel_vec = vec![proto_channel.clone()];
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),8080);
    let proto_server =  Server {
            channels: channel_vec,
            connected_users: user_vec,
            server_name: "ProtoServer".to_string(),
            socket_addr: socket
        };

        proto_server
    }
    pub fn register_client(mut self, client: Client, server: Server) -> ServerReply {
        let nick = &client.user.nickname;
        let user_info = &client.user.full_name;
        let username = client.user.full_name.clone();
        let reg_msg = RegisteredClient {
            user_nick: nick.to_string(),
        };

        let srp = ServerReply {
            prefix: server.server_name,
            num_code: 001,
            param_one: nick.to_string(),
            param_two: "Welcome to the Internet Relay Network <".to_string()
                + &client.user.nickname.to_string()
                + ">!<"
                + &username,
        };
        for ele in &self.connected_users {
            // Refactor to use contains method, if possible.
            if ele.nickname == reg_msg.user_nick {
                return ServerReply {
                    prefix: ":".to_string(),
                    num_code: 433,
                    param_one: ele.nickname.clone(),
                    param_two: ERR_NICKNAMEINUSE.to_string(),
                };
            } else {
                return srp;
            }
        }
        self.connected_users.push(client.user);

        srp
    }

}

#[derive(Clone, Debug)]
pub struct User {
    nickname: String,
    client: IrcClient,
    full_name: String,
}

impl User {
    pub fn send_message(&self, msg: Message) -> Message {
        let msg_len = msg.content.len();
        let max_msg_len = MAX_LEN as usize + CR_LF.len();
        
        if msg_len > max_msg_len {
            panic!("Message over character limit")
        }
        
        let user_msg = Message {
            recipient: "Sender".to_string(),
            sender: "Sender".to_string(),
            command: "".to_string(),
            command_params: "".to_string(),
            content: msg.content.clone(),
        };
        
        
        user_msg
    }
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


#[derive(Clone)]
pub struct Message {
    sender: String,
    recipient: String,
    command: String,
    command_params: String,
    content: String,
}

#[derive(Clone, Debug)]
struct ChannelCreator(String);
#[derive(Clone, Debug)]
struct Channel {
    name: String,
    users: Vec<User>,
    creator: ChannelCreator,
}

#[derive(Clone)]
struct Client {
    server: Server,
    user: User,
}

#[derive(Clone)]
struct ServerReply {
    prefix: String,
    num_code: u32,
    param_one: String,
    param_two: String,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {:?}, {}", self.nickname, self.client, self.full_name)
    }
}


fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
    }

    println!("Hello, world!");
}

