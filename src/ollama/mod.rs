use ollama_rs::Ollama;
use shuttle_runtime::SecretStore;

pub mod chat;

pub fn init_ollama(secret_store: &SecretStore) -> Ollama {
    let ollama_url = secret_store.get("OLLAMA_URL").unwrap();
    let ollama_port = secret_store
        .get("OLLAMA_PORT")
        .unwrap()
        .parse::<u16>()
        .unwrap();

    Ollama::new_with_history(ollama_url, ollama_port, 30)
}
