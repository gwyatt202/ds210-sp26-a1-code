use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

// Your solution solution.
pub struct SolutionAgent {}

// Put your solution here.
impl Agent for SolutionAgent {
// Should returns (<score>, <x>, <y>)
// where <score> is your estimate for the score of the game
// and <x>, <y> are the position of the move your solution will make.
fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
// If you want to make a recursive call to this solution, use
// `SolutionAgent::solve(...)`
 next_move = minimax(board, player);
 next_move; //returning the move that the minimax algorithm generated
    }
}

// minimax impl
// mut ref to board AND player, score of move being tested output
fn minimax(board: &mut Board, player: Player) -> (i32, usize, usize) {

// BASE CASE: game is over, no move to return
// game_over returns a boolean that tells us if the game is over or not (if there are no elements in move vector
// or if it's a full board and the score is not 0)
if board.game_over() { //if true
    return (board.score(), 0, 0); //outputing the board score and a null coord to satisfy output type
} 

// if game continues, we need to find the best move
//initialising best move variable
let avail_moves = board.moves(); // vector with available moves
let mut best_coord = avail_moves[0]; //placeholder for future


let mut best_score = match player { //initialising a best_score for both the X and O
        Player::X => i32::MIN,// maximizer starts at -inf, picked this config cuz aligns with board.rs definitions ( we are X basically )
        Player::O => i32::MAX,// minimizer starts at +inf
};

for coord in avail_moves {
    board.apply_move(coord, player); //applying first iteration of avail_moves so board changes
    let (score, _, _) = minimax(board, player.flip()); // function recursviely called, new available moves and board
// will hopefully go down every single branch after coord has been applied
//so player 1 puts coord, minimax called again so the new board and alternate player:
//checks if game over and returns if yes, gets the available moves
//applies coord to new board
//my turn again
//recurses until the game over
//SO REGARDLESS IF WE ARE X OR 0 WE GO DOWN THE TREES ALTERNATING BETWEEN X AND 0 AND GET TO -1/+1/0
    board.undo_move(coord, player); // undo entire board playout, try the second coord in the initial set of available moves
// now that board is cleared after ever score is recorded, we can record the score of each available move at this current board state
}
}