use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV1 {
    model: Llama,
}

impl ChatbotV1 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV1 { //executes when launched for first time to constuct chatbot
        return ChatbotV1 { model: model };
    }

    #[allow(dead_code)] 
    pub async fn chat_with_user(&mut self, message: String) -> String { //async is large function so lets subsequent functions run asw
        //takes 2 arguements, one for the chatbot and another for the user msg to chatbot
        let mut chat_session: Chat<Llama> = self.model //selects the model
            .chat()
            .with_system_prompt("The assistant will act like a Vietnamese child");
        let asynchronous_output = chat_session.add_message(message);
        let output = asynchronous_output.await;
        
        return output.unwrap();
        
        return String::from("Hello, I am not a bot (yet)!");

}
}