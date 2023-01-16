pub mod game_functions;

use game_functions::*;

fn main() {
    let mut game_field : Vec<char> = vec![' ';9];
    loop{
        //first user [x] turn
        print_field(&game_field);
        let x_turn = get_field_index('x',&game_field);
        to_field(&mut game_field, x_turn-1, 'x');
        
        if is_game_over(&game_field) == true{
            println!("x wins");
            return; 
        }
        if is_game_field_filled(&game_field) == true{
            println!("tie!");
            return;
        }


        //second user [o] turn
        print_field(&game_field);
        let o_turn = get_field_index('o',&game_field);
        to_field(&mut game_field, o_turn-1, 'o');
        
        if is_game_over(&game_field) == true{
            println!("o wins");
            return; 
        }
        if is_game_field_filled(&game_field) == true{
            println!("tie!");
            return;
        }
    }
}