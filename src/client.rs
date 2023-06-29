use std::net::{TcpStream, SocketAddr};
use crate::server::LeafServer;
use crate::user::User;
use std::io::{Read, Write};
use bincode::options;
#[derive(Clone, Debug)]
pub struct Client {
    pub server: Option<Vec<LeafServer>>,
    pub user: User,
}

pub enum Command {
    Connect {
        user_nick: String,
        user_full_name: String,

    },
    Join,
    Part,
    PrivMsg
}

impl Client {

    pub fn new(server: Option<Vec<LeafServer>>, user: User) -> Self {
        Client {
            server: None,
            user
        }
    }

    pub fn register_client(&self, user: &User, socket: SocketAddr, pass: bool) -> Command {

        let mut connection = TcpStream::connect(socket).unwrap();

        let nick_msg = String::from("NICK ") + &user.nickname + " ";
        let user_msg = String::from("USER ") + &user.nickname + " * * " + &user.full_name;


        connection.write_all(nick_msg.as_bytes()).expect("Failed to write.");

        
        connection.write_all(user_msg.as_bytes()).expect("Failed to write.");
        
        let mut buffer = Vec::new();

        connection.read_to_end(&mut buffer).unwrap();

        let server_resp = String::from_utf8(buffer).unwrap();

        
        println!("Server response: {:}", server_resp);

        Command::Connect {
            user_nick: nick_msg,
            user_full_name: user_msg
        }
        
    }

}

#[derive(Debug)]
pub struct ClientMessage(pub String);