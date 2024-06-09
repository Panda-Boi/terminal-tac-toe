pub fn minimax(gamestate: [u8; 9], x_turn: bool, depth: i8) -> i8 {

    //base case, return winner / draw
    if check_state(gamestate.clone(), depth) != 3 {
        return check_state(gamestate.clone(), depth) + depth;
    }

    let mut value;

    if x_turn {
        //maximiser

        value = -100;

        for i in 0..9 {
            //discard positions not empty
            if gamestate[i] != 0 {continue;}

            //get value of the next board if current move is played
            let mut next_gamestate = gamestate.clone();
            next_gamestate[i] = 1; //X is 1
            let next_value = minimax(next_gamestate, false, depth+1);

            //if current move leads to a better board than previous best board, change best board
            if next_value > value {
                value = next_value;
            }
            
        }


    } else {
        //minimiser

        value = 100;

        for i in 0..9 {
            //discard positions not empty
            if gamestate[i] != 0 {continue;}

            //get value of the next board if current move is played
            let mut next_gamestate = gamestate.clone();
            next_gamestate[i] = 2; //O is 2
            let next_value = minimax(next_gamestate, true, depth+1);

            //if current move leads to a better board than previous best board, change best board
            if next_value < value {
                value = next_value;
            }
            
        }

    }

    return value;
        
}

fn check_state(gamestate: [u8; 9], depth: i8) -> i8 {

    //checking the rows and columns
    for i in 0..3 {
        let n = i * 3;

        if gamestate[n] ==  gamestate[n+1] && gamestate[n] == gamestate[n+2] && gamestate[n] != 0{
            if gamestate[n] == 1 {
                return 10;
            }else if gamestate[n] == 2{
                return -10;
            }
        }

        if gamestate[i] == gamestate[i+3] && gamestate[i] == gamestate[i+6] && gamestate[i] != 0{
            if gamestate[i] == 1 {
                return 10;
            }else if gamestate[i] == 2{
                return -10;
            }
        }
    }

    //checking the diagonals
    if gamestate[0] == gamestate[4] && gamestate[0] == gamestate[8] && gamestate[0] != 0 {
        if gamestate[0] == 1 {
            return 10;
        }else if gamestate[0] == 2{
            return -10;
        }
    }
    if gamestate[2] == gamestate[4] && gamestate[2] == gamestate[6] && gamestate[2] != 0{
        if gamestate[2] == 1 {
            return 10;
        }else if gamestate[2] == 2{
            return -10;
        }
    }

    //check for a draw
    if depth >= 8 {
        return 0;
    }

    //3 for neither draw nor a winner
    return 3;
}