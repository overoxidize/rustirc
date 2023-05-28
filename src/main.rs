use core::fmt;
use std::fmt::Error;


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

#[derive(Clone, Debug)]
pub enum IrcClient {
    MIrc,
    FooIrc
}

#[derive(Clone)]
struct Server {
    channels: Vec<Channel>,
    connected_users: Vec<User>,
    pub registered_clients: Vec<RegisteredClient>,
    server_name: String,
}

impl Server {
 fn new(channels: Vec<Channel>, 
    connected_users: Vec<User>, 
    registered_clients: Vec<RegisteredClient>,
    server_name: String) {
    let proto_user = User {
        nickname: "ProtoUser".to_string(),
        client: IrcClient::FooIrc ,
        full_name: "Proto User".to_string()
    };
    //     Server {

    //     }
    }
}
#[derive(Clone)]
struct RegisteredClient {
    user_nick: String,
    user_info: String,
}
#[derive(Clone)]
struct Network {
    servers: Vec<Server>,
}

// pub trait Reply {
//     fn reply(&self) -> Self;
// }

#[derive(Clone)]
pub struct Message {

    sender: String,
    recipient: String,
    command: String,
    command_params: String,
    content: String,
}

enum PrivilegeLevel {
    One,
    Two,
}

enum PrivilegedAction {}

#[derive(Clone)]
struct Channel {
    name: String,
    users: Vec<User>,
    creator: User,
}

#[derive(Clone)]
struct Client {
    server: Server,
    user: User,
}

impl Client {
    // let server = &self.Server;
}

#[derive(Clone)]
struct ServerReply {
    prefix: String,
    num_code: u32,
    param_one: String,
    param_two: String,
}
struct Service;

#[derive(Clone)]
pub struct User
{
    nickname: String,
    client: IrcClient,
    full_name: String,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {:?}, {}", self.nickname, self.client, self.full_name)
    }
}



impl Server {
    pub fn register_client(mut self, client: Client, server: Server) -> ServerReply {
        let nick = &client.user.nickname;
        let user_info = &client.user.full_name;
        let username = client.user.full_name.clone();
        let reg_msg = RegisteredClient {
            user_nick: nick.to_string(),
            user_info: user_info.to_string(),
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
        for ele in &self.registered_clients {
            if ele.user_nick == reg_msg.user_nick {
                return ServerReply {
                    prefix: ":".to_string(),
                    num_code: 433,
                    param_one: ele.user_nick.clone(),
                    param_two: ERR_NICKNAMEINUSE.to_string(),
                };
            } else {
                return srp;
            }
        }
        self.registered_clients.push(reg_msg.clone());

        srp
    }
}

impl User {
    pub fn send_message(&self, msg: Message, recipient: Recipient) -> Message {
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

        match recipient {
            Recipient::Server => {}

            Recipient::User => {}
        }

        user_msg
    }
}

struct Greeting {
    name: String,
}

impl Greeting {
    fn new<T: AsRef<str>>(name: T) -> Self {
        // AsRef is used for computationally non-expensive *reference to reference* conversion
        Greeting {
            name: name.as_ref().to_string()
        }
    }
}

impl fmt::Display for Greeting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hello, {}", self.name)
    }
}
// pub trait Service

fn main() {
    println!("Hello, world!");
}
