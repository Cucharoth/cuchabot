use crate::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use serde_json::Deserializer;
pub use chatgpt::prelude::*;
pub use chatgpt::types::CompletionResponse;
//use poise::serenity_prelude::AttachmentType::File;
use poise::serenity_prelude::json::prelude::to_string;
use poise::serenity_prelude::MessageId;
use crate::Data;


pub struct ChatGptBuilder;

impl ChatGptBuilder {
    pub async fn new(imput: &str, data: &Data, source_id: u64, is_thread: bool) -> Result<(String)> {
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
            .await?;*/
        /*let text = std::fs::read_to_string("my_conversation.json").unwrap();
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
            let mut conversation: Conversation = {
                let mut hash_map = data.discord_thread_info.lock().unwrap();
                let conversation: &Conversation = hash_map.get_mut(&source_id).expect("Did not find id inside discord_thread_info");
                let conversation_aux = Conversation::new_with_history(client, conversation.history.clone());
                conversation_aux
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
            {
                let mut hash_map = data.discord_thread_info.lock().unwrap();
                hash_map.insert(source_id, conversation);
            }
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