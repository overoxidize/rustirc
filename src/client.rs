use std::net::TcpStream;
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