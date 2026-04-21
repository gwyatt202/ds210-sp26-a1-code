use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;
use tic_tac_toe_stencil::board::Cell;


// Your solution solution.
pub struct SolutionAgent {}

// Put your solution here.
impl Agent for SolutionAgent {
    // Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        
        let next_move = minimax(board, player, 5, i32::MIN, i32::MAX);
        return next_move; 
    }
}


//implementint our scoring function
fn score_move(board: &mut Board, muve: (usize, usize), player: Player) -> i32 { 
        //score the available moves based on heuristic 
        board.apply_move(muve, player);
        let score = board.score();
        board.undo_move(muve, player);
        match player {
            Player::X => -score,
            Player::O => score,
        }
    }

//OUR WINDOW
fn score_window(a: &Cell, b: &Cell, c: &Cell) -> i32 { // window of 3 rewards
    match (a, b, c) {
        // complete 3 in a row
        (Cell::X, Cell::X, Cell::X) => 500,
        (Cell::O, Cell::O, Cell::O) => -500,

        // open ended 2 in a row
        (Cell::X, Cell::X, Cell::Empty) => 100,
        (Cell::Empty, Cell::X, Cell::X) => 100,
        (Cell::O, Cell::O, Cell::Empty) => -100,
        (Cell::Empty, Cell::O, Cell::O) => -100,

        // gap pattern
        (Cell::X, Cell::Empty, Cell::X) => 80,
        (Cell::O, Cell::Empty, Cell::O) => -80,

        // blocked one side
        (Cell::X, Cell::X, Cell::Wall) => 30,
        (Cell::Wall, Cell::X, Cell::X) => 30,
        (Cell::O, Cell::O, Cell::Wall) => -30,
        (Cell::Wall, Cell::O, Cell::O) => -30,

        // dead sequences
        (Cell::X, Cell::X, Cell::O) => 0,
        (Cell::O, Cell::X, Cell::X) => 0,
        (Cell::O, Cell::O, Cell::X) => 0,
        (Cell::X, Cell::O, Cell::O) => 0,

        _ => 0,
        }
    }

//HEROISTIC!!!!!
fn heuristic(board: &Board) -> i32 {
    
    let mut score = board.score() * 1000; // weight actual completed defined sequences heavily

    //center cell bias
    let center = board.get_cells().len() / 2; // for 5x5 this is 2
    for i in 0..board.get_cells().len() {
        for j in 0..board.get_cells().len() {
            let cell = &board.get_cells()[i][j];
            // distance from center
            let dist = (i as i32 - center as i32).abs() + (j as i32 - center as i32).abs();
            let center_bonus = match cell {
                Cell::X => 10 - (dist * 2),  // closer to center = higher bonus
                Cell::O => -(10 - (dist * 2)), // opposite for O
                _ => 0,
            };
            score += center_bonus;
        }
    }

    for i in 0..board.get_cells().len() {
            for j in 0..board.get_cells().len() {
                // Count row.
                if j + 2 < board.get_cells().len() {
                    let x = &board.get_cells()[i][j];
                    let y = &board.get_cells()[i][j + 1];
                    let z = &board.get_cells()[i][j + 2];
                    score += score_window(x, y, z);
                }
                // Count col.
                if i + 2 < board.get_cells().len() {
                    let x = &board.get_cells()[i][j];
                    let y = &board.get_cells()[i + 1][j];
                    let z = &board.get_cells()[i + 2][j];
                    score += score_window(x, y, z);
                }
                // 1st diagonal
                if i + 2 < board.get_cells().len() && j + 2 < board.get_cells().len() {
                    let x = &board.get_cells()[i][j];
                    let y = &board.get_cells()[i + 1][j + 1];
                    let z = &board.get_cells()[i + 2][j + 2];
                    score += score_window(x, y, z);
                }

                // 2nd diagonal
                if i + 2 < board.get_cells().len() && j >= 2 {
                    let x = &board.get_cells()[i][j];
                    let y = &board.get_cells()[i + 1][j - 1];
                    let z = &board.get_cells()[i + 2][j - 2];
                    score += score_window(x, y, z);
                }
            }
        }

        return score;
    
}





//MINIMAX FUNCTION CALL
fn minimax(board: &mut Board, player: Player, depth: u32, mut alpha: i32, mut beta: i32) -> (i32, usize, usize) {  

    if depth == 0 || board.game_over() { //depth is 0 or game over
    return (heuristic(board), 0, 0);
}


    let mut avail_moves = board.moves();
    avail_moves.sort_by_key(|&muve| score_move(board, muve, player));
    //rearranging available moves vector in concordance with score heirarchy
    let mut best_coord = avail_moves[0]; 
    // initialising available moves vector and best coordinate 


    let mut best_score = match player { 
        Player::X => i32::MIN,  
        Player::O => i32::MAX,  
    };
    //best score is applied to both X (maximiser) and 0 (minimiser) using match 


    //recursion begins
    for coord in avail_moves {
        board.apply_move(coord, player); 
        let (score, _, _) = minimax(board, player.flip(), depth - 1, alpha, beta);
        //whenever a terminal state is reached, score variable has a value from minimax function
        board.undo_move(coord, player);
                //condition for the either player to get the best score
                match player {
                Player::X => {            
                    if score > best_score { 
                        best_coord = coord; 
                        best_score = score; 
                    }
                    alpha = best_score; //X's best score so far 
                    if alpha >= beta { //if true, reached alpha so good that 0 would never pick it
                        break; //force stop exploring, 0 GAURENTEED TO EXPLORE ANOTHER BRANCH
                    }
                }
                Player::O => { 
                    if score < best_score { 
                        best_coord = coord;
                        best_score = score;
                    }
                    beta = best_score;
                    if alpha >= beta { //if true, found beta that is so low that X would never let 0 choose this path
                        break; //force stop exploring, down this branch because another branch is gaurenteed to be EXPLORES BY X
                    }
                }
            }
    };   
  return (best_score, best_coord.0, best_coord.1);
}




