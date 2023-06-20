use std::net::{TcpStream, SocketAddr};
use crate::server::Server;
use crate::user::User;
use std::io::{Read, Write};

#[derive(Clone, Debug)]
pub struct Client {
    pub server: Server,
    pub user: User,
}

impl Client {

    pub fn new(server: Server, user: User) -> Self {
        Client {
            server,
            user
        }
    }

    pub fn register_client(&self, user: &User, socket: SocketAddr, pass: bool) {

        let mut connection = TcpStream::connect(socket).unwrap();

        let nick_msg = String::from("NICK") + &user.nickname;
        let user_msg = String::from("USER") + &user.full_name;


        connection.write(nick_msg.as_bytes()).expect("Failed to write.");

        
        connection.write(nick_msg.as_bytes());
        connection.write(user_msg.as_bytes());
        
        let mut buffer = Vec::new();

        connection.read_to_end(&mut buffer).unwrap();

        let server_resp = String::from_utf8(buffer).unwrap();

        
        println!("Server response: {:}", server_resp);

        
    }

}