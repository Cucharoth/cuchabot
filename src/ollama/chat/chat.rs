
use std::sync::Arc;

use ollama_rs::error::OllamaError;
use ollama_rs::generation::chat::{request::ChatMessageRequest, ChatMessage};
use tracing::error;

use crate::Db;

pub struct OllamaChat;

impl OllamaChat {
    pub async fn init_chat(data: &Arc<Db>, input: String) -> Result<String, OllamaError> {
        let mut ollama = {
            let mutex = data.ollama.lock();
            mutex.unwrap().clone()
        };

        let model = "cuchabot".to_string();

        let user_message = ChatMessage::user(input);

        let response = ollama
            .send_chat_messages_with_history(
                ChatMessageRequest::new(model, vec![user_message]),
                "default",
            )
            .await?;
        Ok(response.message.unwrap().content)
    }

    pub async fn new_score_interaction(
        data: &Arc<Db>,
        username: &str,
        old_pp: f32,
        new_pp: f32,
    ) -> String {
        let pp_gain = new_pp - old_pp;
        let prompt = format!(
            "{username} just got a new score and won {pp_gain:.1} pp and sits now with a total of {new_pp:.1} pp, give him some congratulations, mention {username}'s name",
            username = username,pp_gain = pp_gain,new_pp = new_pp
        );

        match Self::init_chat(data, prompt).await {
            Ok(bot_response) => bot_response,
            Err(why) => {
                error!("ollama error: {}", why);
                format!("Congrats on your new pp record!, you sit at: {}pp", new_pp)
            }
        }
    }
}
