use kalosm::language::*;
use file_chatbot::solution::file_library::{self, load_chat_session_from_file, save_chat_session_to_file};

use crate::solution::Cache;

pub struct ChatbotV5 {
    model: Llama,
    cache: Cache<Chat<Llama>>,
}

impl ChatbotV5 {
    pub fn new(model: Llama) -> ChatbotV5 {
        return ChatbotV5 {
            model: model,
            cache: Cache::new(3),
        };
    }

    //std 2 phil
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);

        match cached_chat {
            None => {
                println!("chat_with_user: {username} is not in the cache!");
                // The cache does not have the chat. What should you do?
                //check if there is file saved using is_some()
                let file_check = load_chat_session_from_file(filename);
                if file_check.is_some() {
                //if true, add session that that file
                let mut old_session = self.model.chat().with_session(file_check.unwrap());
                let chat_session = old_session.add_message(message);
                let output = chat_session.await.unwrap();
                save_chat_session_to_file(filename, &old_session.session().unwrap());
                self.cache.insert_chat(username, old_session);
                return output;

                } else {  // if false, initialise a new chat 
                let mut initialise_chat = self.model.chat().with_system_prompt("The assistant will act like donald trump");
                let chat_session = initialise_chat.add_message(message);
                let output = chat_session.await.unwrap();
                save_chat_session_to_file(filename, &initialise_chat.session().unwrap());
                self.cache.insert_chat(username, initialise_chat);

                return output;

                }
                return String::from("Hello, I am not a bot (yet)!");
            }
            Some(chat_session) => {
                println!("chat_with_user: {username} is in the cache! Nice!");
                //just return the chat_session in cache and add message from there
                let output = chat_session.add_message(message).await.unwrap();
                save_chat_session_to_file(filename, &chat_session.session().unwrap());
                return output;
                return String::from("Hello, I am not a bot (yet)!")

            }
        }
    }

    //std 1 george
    pub fn get_history(&mut self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);

        match cached_chat {
            None => {
                println!("get_history: {username} is not in the cache!");
                //check if username exists in harddrive using load function
                if load_chat_session_from_file(filename).is_some() {
                    let old_session_maybe = load_chat_session_from_file(filename);
                    let chat_session = old_session_maybe.unwrap();
                    let history = &chat_session.history()[1..];
                    let stuff_in_history = history.iter().map(|msg| String::from(msg.content()));
                    // extracting the strings in history by iterating and for each msg(iteration), and gives string ownership to variable
                    let final_contents = stuff_in_history.collect();
                    //pushing the strings into a vector
                    return final_contents;
                } else {
                    // if not, create new vec
                    return Vec::new();
                }
                
            }
            Some(chat_session) => {
                println!("get_history: {username} is in the cache! Nice!");
                let session = chat_session.session().unwrap();
                //turn the LLM into LLM session and unwrap to match types
                let history = &session.history()[1..];
                // removing the first promt
                let stuff_in_history = history.iter().map(|msg| String::from(msg.content()));
                // extracting the strings in history by iterating and for each msg(iteration), and gives string ownership to variable
                let final_contents = stuff_in_history.collect();
                //pushing the strings into a vector
                return final_contents;
                //return the contents in the chat_session
                return Vec::new();

            }
        }
    }
}