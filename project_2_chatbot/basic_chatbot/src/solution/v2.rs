use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV2 {
    // What should you store inside your Chatbot type?
    // The model? The chat_session?
    model: Llama,
    chat_session: Chat<Llama>, //chatbotv2 contains these 2 objects now (we can mess with chat_session in the new() function)
}

impl ChatbotV2 {
    //putting functions into the objects in struct
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV2 {
        //executes when launched for first time to constuct chatbot

        let chat_session = model
            //selects the model and starts the chat session during model initialisation
            .chat()
            .with_system_prompt("The assistant will act like a really annoying child");

        return ChatbotV2 {
            model: model,
            chat_session,
            //returns both objects from the ChatbotV2 struct (model and chat_session)
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, message: String) -> String {
        let asynchronous_output = self.chat_session.add_message(message);
        //specify which object in the struct to call to .add_message
        let output = asynchronous_output.await;

        return output.unwrap(); //extracts the success value, or crashes the program if there's an error
        // Add your code for chatting with the agent while keeping conversation history here.
        // return String::from("Hello, I am not a bot (yet)!");
    }
}