use kalosm::language::*;

// Look at the docs for std::fs
// https://doc.rust-lang.org/std/fs/index.html
// std::fs provides functions that write to a file, read from a file,
// check if a file exists, etc.
use std::fs;

// LlamaChatSession provides helpful functions for loading and storing sessions.
// Look at https://docs.rs/kalosm/latest/kalosm/language/trait.ChatSession.html#saving-and-loading-sessions
// for some examples!

 // Student 2 (phil)
    // Implement this
    pub fn load_chat_session_from_file(filename: &str) -> Option<LlamaChatSession> {
    // look at fs::read(...)
    // also look at LlamaChatSession::from_bytes(...)
    unimplemented!("Loading chat session from file {filename}");
    }
// Student 1 (george)
    pub fn save_chat_session_to_file(filename: &str, session: &LlamaChatSession) {
    // converts the ChatSession object back to bytes
    let bytes = session.to_bytes().unwrap();  // Crash if fails
    fs::write(filename, bytes).unwrap();
    //creates a new file on laptop, don't need output for function

    }

    //v3 takes orders of magnitude less time to do get_history whereas v4 takes a lot longer. 
    //this because reading and deserialising takes a long time
    //that is because reading from hard drive takes longer that reading from memory