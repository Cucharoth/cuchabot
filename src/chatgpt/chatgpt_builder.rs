use std::borrow::Borrow;
use crate::prelude::*;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, BufReader, BufWriter};
use std::io::Write;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
pub use chatgpt::prelude::*;
pub use chatgpt::types::CompletionResponse;
//use poise::serenity_prelude::AttachmentType::File;
use poise::serenity_prelude::json::prelude::to_string;
use poise::serenity_prelude::{MessageId, User};
use crate::Data;


pub struct ChatGptBuilder;

impl ChatGptBuilder {
    pub async fn new(imput: &str, data: &Data, source_id: u64, user: &User,  is_thread: bool) -> Result<(String)> {
        dotenv().ok();
        //chatgpt client builder
        let key = env::var("CHATGPT_KEY").expect("key not found");
        let client = ChatGPT::new_with_config(
            key,
            ModelConfigurationBuilder::default()
                .engine(ChatGPTEngine::Gpt35Turbo)
                .temperature(0.5)
                .build()
                .unwrap(),
        )?;

        //keeping conversation with discord thread
        let response: CompletionResponse = if is_thread {
            let mut conversation: Conversation = {
                /*
                let mut hash_map = data.discord_thread_info.lock().unwrap();
                let conversation: &Conversation = hash_map.get_mut(&source_id).expect("Did not find id inside discord_thread_info");
                let conversation_clone = Conversation::new_with_history(client, conversation.history.clone());
                conversation_clone*/
                let mut bst = data.thread_info_as_bst.lock().unwrap();
                let data_tuple: Arc<(User, Conversation)> = bst.get_by_key(source_id).expect("Did not find ID inside bst");
                let conversation = &data_tuple.1;
                let conversation_clone = Conversation::new_with_history(client, conversation.history.clone());
                conversation_clone
            };
            let response: CompletionResponse = conversation
                .send_message(imput)
                .await?;
            response
        } else {
            let direction = "You are CuchaBot a Discord bot. Answer as concisely as possible";
            let mut conversation: Conversation = client.new_conversation_directed(direction);
            let response: CompletionResponse = conversation
                .send_message(imput)
                .await?;
            /*{
                let mut hash_map = data.discord_thread_info.lock().unwrap();
                hash_map.insert(source_id, conversation);
            }*/
            {
                let mut bst = data.thread_info_as_bst.lock().unwrap();
                bst.insert(source_id, (user.clone(), conversation))
            }
            Self::write_conversation_log(&user, source_id);
            response
        };

        //keeping conversation with Json
        /*let mut restored = client
            .restore_conversation_json("my_conversation.json")
            .await?;
        let text = std::fs::read_to_string("my_conversation.json").unwrap();
        let mut conversation: Conversation = if text == "" {
            let direction = "You are CuchaBot a Discord bot. Answer as concisely as possible";
            client.new_conversation_directed(direction)
        } else{
            let mut restored = client
                .restore_conversation_json("my_conversation.json")
                .await?;
            Conversation::new_with_history(client, restored.history)
        };
        let response: CompletionResponse = conversation
            .send_message(imput)
            .await?;
        conversation.save_history_json("my_conversation.json").await?;
        */

        let respuesta = &response.message().content;

        Ok(respuesta.to_string())
    }

    fn write_conversation_log(user: &User, source_id: u64) {
        let file_path = "my_conversation.txt";
        if let Ok(mut file) = OpenOptions::new()
            .append(true)
            .open(file_path) {
            writeln!(file, "conversation ID: {} | User: {:?}", source_id, user).expect("TODO: panic message");
        }
    }

    pub fn read_logs(){
        let file_path = "my_conversation.txt";
        match Self::read_lines(file_path) {
            Ok(lines) => {
                for line in lines {
                    if let Ok(whole_line) = line {
                        println!("{}", whole_line);
                    }
                }
            },
            Err(error) => panic!("No se pudo acceder al archivo!"),
        }
    }

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    pub fn reset(){
        if let Ok(mut file) = File::create("my_conversation.json") {}
    }
}