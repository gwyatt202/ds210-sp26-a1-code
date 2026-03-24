use kalosm::language::*;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct ChatbotV3 {
    // What should you store inside your Chatbot type?
    // The model? The chat_session?
    // Storing a single chat session is not enough: it mixes messages from different users
    // together!
    // Need to store one chat session per user.
    // Think of some kind of data structure that can help you with this.
    model: Llama,
    sessions: HashMap<String, Chat<Llama>>,
}

impl ChatbotV3 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV3 {
        return ChatbotV3 {
            // Make sure you initialize your struct members here
            model,
            sessions: HashMap::new(),
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        // Add your code for chatting with the agent while keeping conversation history here.
        // Notice, you are given both the `message` and also the `username`.
        // Use this information to select the correct chat session for that user and keep it
        // separated from the sessions of other users.


        // Check if user exists
        if self.sessions.contains_key(&username) {
        // if true, then pass
        } else {
        // User does not exist, initialise a new session
        let new_session = self.model
            .chat()
            .with_system_prompt("The assistant will act like a really annoying child");

        //this inserts the new name into the new session initialised above. 
        //must clone the username so the username can be recalled, otherwise it would be MOVED (reference changes)
        self.sessions.insert(username.clone(), new_session); //new session is in self.session

        }
        //get the reference to the session that is called by the &username.
        //need get_mut so we can change the sessions 
        //(remember hashmap when call key, we get the value, and we want to change the value)
        let chat_session = self.sessions.get_mut(&username).unwrap();
        
        //same as v2 here
        let asynchronous_output = chat_session.add_message(message); 
        //specify which object in the struct to call to .add_message
        let output = asynchronous_output.await;
        return output.unwrap(); //extracts the success value, or crashes the program if there's an error

        return String::from("Hello, I am not a bot (yet)!");
    }





    #[allow(dead_code)]
    pub fn get_history(&self, username: String) -> Vec<String> { // &self allows reference to the CHATBOTv3 struct (model and sessions)
       if let Some(chat_session) = self.sessions.get(&username) {  //chat_session IS A REFERENCE TO THE VALUE OF STRUCT FROM CHATBOTV3
        //shorthand for:
        //match self.sessions.get(&username) 
        //Some (chat_session) 
        //so if the username exists in the struct, we run the below code, if not, create a new vector
        //makes the variable chat_session which is a reference to the chatbot in the struct
            return chat_session.session().unwrap().history() //takes the session object from chat_session, reads message
                .iter() //loop thu each msg
                .filter(|m| m.role() != MessageType::SystemPrompt) //keeps only user and chatbot msgs
                .map(|m| m.content().to_string()) //extracts text from each msg
                .collect(); //gathers all strings into a vec<str>
        }
        return Vec::new(); //what happens if the condition fails
    }
}

