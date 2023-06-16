use std::net::{TcpListener, TcpStream, SocketAddr, IpAddr, Ipv4Addr};
use std::{io, time};
use std::io::{prelude::*, BufReader};
use std::thread;
use uuid::Uuid;


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

#[derive(Clone, Debug)]
pub enum IrcClient {
    MIrc,
    FooIrc
}

#[derive(Clone, Debug)]
struct Server {
    channels: Vec<Channel>,
    pub connected_users: Vec<User>,
    server_name: String,
    socket_addr: SocketAddr,
    nud: Uuid,

}

impl Server {
 fn new(channels: Vec<Channel>, 
    connected_users: Vec<User>, 
    server_name: String,
    socket_addr: SocketAddr,
    nud: Uuid) -> Self {

        if server_name.len() > 63 {
            eprintln!("Server name cannot be greater than 63 characters in length.");
        } 
            Server {
                channels,
                connected_users,
                server_name,
                socket_addr,
                nud
            }

        }

    fn run(self) {
        let listener = TcpListener::bind(self.socket_addr).unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    handle_connection(stream);
                }
                Err(e) => {
                    eprintln!("There was a server error {:?}", e);
                }
            }
        }
    }

    fn handle_registration(mut self, client: Client, user: User) -> ServerReply {
       
        let conn_user_vec: Vec<String> = self.connected_users.iter().map(|user| user.nickname.clone()).collect();

        let srp: ServerReply;

        if conn_user_vec.contains(&user.nickname) {
            srp = ServerReply {
                prefix: ":".to_string(),
                num_code: 433,
                param_one: user.nickname.clone(),
                param_two: ERR_NICKNAMEINUSE.to_string(),
            };

            eprintln!("This username has already been taken");

            return srp
        } else {
            srp = ServerReply {
                prefix: self.server_name,
                num_code: 001,
                param_one: user.nickname.to_string(),
                param_two: "Welcome to the Internet Relay Network <".to_string()
                    + &client.user.nickname.to_string()
                    + ">!<"
                    + &user.full_name,
            };

            srp
        }
    }

    fn handle_sender(mut stream: TcpStream) -> io::Result<()> {
        let mut buf = [0;512];
    
        for _ in 0..1000 {
            let bytes_read = stream.read(&mut buf)?;
    
            if bytes_read == 0 {
                return Ok(())
            }
    
            stream.write(&buf[..bytes_read])?;
    
            println!("from the sender: {}", String::from_utf8_lossy(&buf));
    
            thread::sleep(time::Duration::from_secs(1));
        }
    
        Ok(())
    }
    // let proto_user = User {
    //     nickname: "ProtoUser".to_string(),
    //     client: IrcClient::FooIrc ,
    //     full_name: "Proto User".to_string()
    // };
    // let user_vec = vec![proto_user.clone()];
    // let channel_owner = ChannelCreator(proto_user.nickname.clone());
    // let proto_channel = Channel {
    //     name: "ProtoChannel".to_string(),
    //     users: user_vec.clone(),
    //     creator: channel_owner
        
    // };
    

    // let channel_vec = vec![proto_channel.clone()];
    
    // let proto_server =  Server {
    //         channels: channel_vec,
    //         connected_users: user_vec,
    //         server_name: "ProtoServer".to_string(),
    //         socket_addr: socket
    //     };

    }

fn handle_connection(mut stream: TcpStream) -> io::Result<()>{
    let buf_reader = BufReader::new(&mut stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

        let response = "HTTP/1.1 200 OK\r\n\r\n";

        stream.write_all(response.as_bytes()).unwrap();

        Ok(())
}

#[derive(Clone, Debug)]
pub struct User {
    nickname: String,
    client: IrcClient,
    full_name: String,
}

impl User {
    pub fn new(nickname: String, client: IrcClient, full_name: String) -> Self {
        User {
            nickname,
            client: IrcClient::FooIrc,
            full_name,
        }
    }
    pub fn send_message(&self, msg: UserMessage) -> UserMessage {
        let msg_len = msg.content.len();
        let max_msg_len = MAX_LEN as usize + CR_LF.len();
        
        if msg_len > max_msg_len {
            panic!("Message over character limit")
        }
        
        let user_msg = UserMessage {
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
pub struct UserMessage {
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

#[derive(Clone, Debug)]
struct Client {
    server: Server,
    user: User,
}


impl Client {

    pub fn new(server: Server, user: User) -> Self {
        Client {
            server,
            user
        }
    }

    pub fn register_client(mut self, user: User, mut server: Server, pass: bool) {

        let mut connection = TcpStream::connect(server.socket_addr).unwrap();

        let nick_msg = String::from("Nick") + &user.nickname.to_string();
        let user_msg = String::from("Nick") + &user.nickname.to_string();
        let mut buffer = Vec::new();

        connection.write(nick_msg.as_bytes());
        connection.write(user_msg.as_bytes());

        connection.read_to_end(&mut buffer);

        let server_resp = String::from_utf8(buffer).unwrap();

        // let nick = &user.nickname;
        // let user_info = &user.full_name;
        // let username = user.full_name;
        // let reg_msg = RegisteredClient {
        //     user_nick: nick.to_string(),
        // };

        
        println!("Server response: {:}", server_resp);
            
        server.connected_users.push(user);

        
    }

}

#[derive(Clone)]
struct ServerReply {
    prefix: String,
    num_code: u32,
    param_one: String,
    param_two: String,
}



fn handle_sender(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0;512];

    for _ in 0..1000 {
        let bytes_read = stream.read(&mut buf)?;

        if bytes_read == 0 {
            return Ok(())
        }

        stream.write(&buf[..bytes_read])?;

        println!("from the sender: {}", String::from_utf8_lossy(&buf));

        thread::sleep(time::Duration::from_secs(1));
    }

    Ok(())
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

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

    let channel_vec = vec![proto_channel];

    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),8080);
    let server_nud = Uuid::new_v4();
    let server = Server {
        channels: channel_vec,
        connected_users: user_vec,
        server_name: String::from("ProtoServer"),
        socket_addr: socket,
        nud: server_nud
    };
    // Either within the function `run` itself, or prior to calling it,
    // we need to set up writing to stdout.
    server.run();
    
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();

    for stream in listener.incoming() {
        let stream = stream.expect("failed");

        let handle = thread::spawn(move || {
            handle_sender(stream).unwrap_or_else(|error| eprintln!("{:?}", error))
        });

        thread_vec.push(handle);
        
    }
    
    for handle in thread_vec {
        handle.join().unwrap();
    }

}

