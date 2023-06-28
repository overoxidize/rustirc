use uuid::Uuid;
use std::fmt::Formatter;
use std::{fmt, collections::HashMap};
use std::net::{SocketAddr, TcpListener, TcpStream};
use crate::channel::Channel;
use crate::user::User;
use crate::client::Client;
use std::{io, time};
use std::io::{Read, Write, BufReader, BufRead};
use std::thread;
use std::sync::Arc;

const ERR_NICKNAMEINUSE: &str = "433";

#[derive(Clone, Debug)]
pub struct ServerName(pub String);

impl fmt::Display for ServerName {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct HubServer {
    pub leaf_servers: Option<HashMap<ServerName, LeafServer>>,
    pub socket_addr: SocketAddr,
    pub nud: Uuid,
}

pub fn hub_run(hub_server: &HubServer) {
    let listener: TcpListener = TcpListener::bind(hub_server.socket_addr).unwrap();
    let hub_server = hub_server.clone();
    thread::spawn(move || {
        for stream in listener.incoming() {


          match stream  {
                Ok(stream) => {
                    let stream = hub_server.handle_registration(stream).unwrap();
                }
                Err(e) => {
                    eprintln!("There was a server error {:?}", e);
                }
            }

        }
    });


}
impl HubServer {

        pub fn handle_registration(&self, mut stream: TcpStream) -> io::Result<()> {
        let mut buf = [0;512];
    
        for _ in 0..1000 {
            let bytes_read = stream.read(&mut buf)?;
    
            if bytes_read == 0 {
                return Ok(())
            }
    
            let client_msg = String::from_utf8(buf.to_vec()).unwrap();
            println!("from the sender: {}", client_msg);
    
            thread::sleep(time::Duration::from_secs(1));
        }
    
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct LeafServer {
    pub channels: Vec<Channel>,
    pub connected_users: Vec<User>,
    pub server_name: ServerName,
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


impl LeafServer {
 fn new(channels: Vec<Channel>, 
    connected_users: Vec<User>, 
    server_name: String,
    socket_addr: SocketAddr,
    nud: Uuid) -> Self {

        if server_name.len() > 63 {
            eprintln!("Server name cannot be greater than 63 characters in length.");
        } 
            LeafServer {
                channels,
                connected_users,
                server_name: ServerName(String::from("")),
                socket_addr,
                nud
            }

        }

    pub fn run(self) {
        let listener = TcpListener::bind(self.socket_addr).unwrap();

        println!("Top of run: {}", 3);

        thread::spawn(move || {

            for stream in listener.incoming() {
                // check out incoming() docs
                match stream {
                    Ok(stream) => {
                        println!("stream: {:?}", stream);
                        &self.handle_sender(&stream).unwrap();
                    }
                    Err(e) => {
                        eprintln!("There was a server error {:?}", e);
                    }
                }
    
            }
        });
        println!("Count inside loop {}", 1);
    }

    pub fn handle_registration(&mut self, client: Client, user: User) -> ServerReply {
       
        let conn_user_vec: Vec<_> = self.connected_users.iter().map(move |user| user.nickname.clone()).collect();

        let srp: ServerReply;

        let mut connection = TcpStream::connect(self.socket_addr).unwrap();

        let mut buffer = Vec::new();

        let client_msg = connection.read_to_end(&mut buffer).unwrap();

        let client_parse = String::from_utf8(buffer).unwrap();

        println!("Client parse: {:?}", client_parse);
        if conn_user_vec.contains(&user.nickname) {
            let srp = ServerReply {
                prefix: ":".to_string(),
                num_code: 433,
                param_one: user.nickname.to_string(),
                param_two: ERR_NICKNAMEINUSE.to_string(),
            };

            eprintln!("This username has already been taken");
            

            return srp
        } else {
            srp = ServerReply {
                prefix: ":".to_string() + &self.server_name.to_string(),
                num_code: 001,
                param_one: user.nickname.to_string(),
                param_two: "Welcome to the Internet Relay Network <".to_string()
                    + &client.user.nickname
                    + ">!<"
                    + &user.full_name,
            };

            // connection.write(srp.param_two.as_bytes()).unwrap();
            self.connected_users.push(user);
            return srp
        }
    }

    pub fn handle_sender(&self, mut stream: &TcpStream) -> io::Result<()> {
        let mut buf = [0;512];
    
        for _ in 0..1000 {
            let bytes_read = stream.read(&mut buf)?;
    
            if bytes_read == 0 {
                return Ok(())
            }
    
            let client_reg_msg = stream.write(&buf[..bytes_read])?;
    
            let client_msg = String::from_utf8(buf.to_vec()).unwrap();
            println!("from the sender: {}", client_msg);
    
            thread::sleep(time::Duration::from_secs(1));
        }
    
        Ok(())
    }

    fn handle_connection(self, mut stream: TcpStream) -> io::Result<()>{
        
        println!("Handle connection output: {}", 2);
        
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

    }
