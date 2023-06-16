const MAX_LEN: u32 = 510;

const CR_LF: &str = "\r\n";

#[derive(Clone, Debug)]
pub struct User {
    pub nickname: String,
    pub client: IrcClient,
    pub full_name: String,
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
pub enum IrcClient {
    MIrc,
    FooIrc
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