use uuid::Uuid;
use std::net::{SocketAddr, TcpListener, TcpStream};
use crate::channel::Channel;
use crate::user::User;
use crate::client::Client;
use std::{io, time};
use std::io::{Read, Write, BufReader, BufRead};
use std::thread;
use std::rc::Rc;
use std::cell::RefCell;
const ERR_NICKNAMEINUSE: &str = "433";

#[derive(Clone, Debug)]
pub struct Server {
    pub channels: Vec<Channel>,
    pub connected_users: Vec<User>,
    pub server_name: Rc<RefCell<String>>,
    pub socket_addr: SocketAddr,
    pub nud: Uuid,

}

#[derive(Clone, Debug)]
pub struct ServerReply {
    prefix: String,
    num_code: u32,
    param_one: String,
    param_two: String,
}


impl Server {
 fn new(channels: Vec<Channel>, 
    connected_users: Vec<User>, 
    server_name: Rc<RefCell<String>>,
    socket_addr: SocketAddr,
    nud: Uuid) -> Self {

        if server_name.borrow().len() > 63 {
            eprintln!("Server name cannot be greater than 63 characters in length.");
        } 
            Server {
                channels,
                connected_users,
                server_name: Rc::new(RefCell::new(String::from(""))),
                socket_addr,
                nud
            }

        }

    pub fn run(&self) {
        let listener = TcpListener::bind(self.socket_addr).unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.clone().handle_connection(stream).unwrap();
                }
                Err(e) => {
                    eprintln!("There was a server error {:?}", e);
                }
            }
        }
    }

    pub fn handle_registration(&mut self, client: Client, user: User) -> ServerReply {
       
        let conn_user_vec: Vec<_> = self.connected_users.iter().map(move |user| user.nickname.clone()).collect();

        let srp: ServerReply;

        if conn_user_vec.contains(&user.nickname) {
            let srp = ServerReply {
                prefix: ":".to_string(),
                num_code: 433,
                param_one: user.nickname.borrow().to_string(),
                param_two: ERR_NICKNAMEINUSE.to_string(),
            };

            eprintln!("This username has already been taken");
            
            let mut connection = TcpStream::connect(self.socket_addr).unwrap();

            srp
        } else {
            srp = ServerReply {
                prefix: self.server_name.borrow().to_string(),
                num_code: 001,
                param_one: user.nickname.borrow().to_string(),
                param_two: "Welcome to the Internet Relay Network <".to_string()
                    + &client.user.nickname.borrow()
                    + ">!<"
                    + &user.full_name,
            };

            self.connected_users.push(user);
            srp
        }
    }

    pub fn handle_sender(self, mut stream: &TcpStream) -> io::Result<()> {
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

    fn handle_connection(self, mut stream: TcpStream) -> io::Result<()>{
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