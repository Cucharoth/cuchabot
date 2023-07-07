use std::borrow::Borrow;
use crate::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
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
        println!("creating chatgpt client");
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


        //keeping conversation with discord thread
        let response: CompletionResponse = if is_thread {
            println!("message inside tread instance");
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
            println!("first message instance.");
            let direction = "You are CuchaBot a Discord bot. Answer as concisely as possible";
            let mut conversation: Conversation = client.new_conversation_directed(direction);
            let response: CompletionResponse = conversation
                .send_message(imput)
                .await?;
            println!("debug: got response");
            /*{
                let mut hash_map = data.discord_thread_info.lock().unwrap();
                hash_map.insert(source_id, conversation);
            }*/
            {
                let mut bst = data.thread_info_as_bst.lock().unwrap();
                bst.insert(source_id, (user.clone(), conversation))
            }
            println!("debug: after first message instance");
            response
        };

        println!("Response: {}", response.message().content);
        let respuesta = &response.message().content;

        Ok(respuesta.to_string())

    }

    pub fn reset(){
        if let Ok(mut file) = File::create("my_conversation.json") {}
    }
}