pub struct ConvertResult{
    index : usize,
    is_convert_error : bool
}

pub fn input()->String{
    let mut result : String = String::new();
    match std::io::stdin().read_line(&mut result){
        Ok(..) => {}
        Err(err_msg) => println!("IO Error : {}",err_msg)
    };
    return result;
}

pub fn str_to_usize(str : &String)->ConvertResult{
    match str.trim().parse::<usize>(){
        Ok(parse_result) => {
            return ConvertResult{index : parse_result,is_convert_error : false};
        },
        Err(err_msg) => {
            println!("Parse error : {}",err_msg);
            return ConvertResult{index : 0,is_convert_error : true};
        }
    }
}

pub fn get_usize()->usize{
    loop{
        let result = str_to_usize(&input());
        if result.is_convert_error == true {
            continue;
        }else {
            return result.index;
        }
    }
}

pub fn get_field_index(xo : char,game_field : &Vec<char>)->usize{
    loop{
        println!("{} turn : ",xo);
        let index = get_usize();
        if index <= 9 && index >= 1 && game_field[index - 1] == ' '{
            return index;
        }
    }
}

pub fn print_field(game_field_array : &Vec<char>){
    println!(" ________");
    println!("| {}| {}| {}|",game_field_array[0],game_field_array[1],game_field_array[2]);
    println!(" ________");
    println!("| {}| {}| {}|",game_field_array[3],game_field_array[4],game_field_array[5]);
    println!(" ________");
    println!("| {}| {}| {}|",game_field_array[6],game_field_array[7],game_field_array[8]);
    println!(" ________");
}

pub fn to_field(game_field : &mut Vec<char>,index : usize,xo : char){
    game_field[index] = xo;
}

pub fn is_game_over(gf : &Vec<char>)->bool{
    //gf - game_field

    //all win combination:
    //012 345 678 036 147 048 246 258

    //check to x win
    if gf[0] == 'x' && gf[1] == 'x' && gf[2] == 'x'{
        return true;
    }
    if gf[3] == 'x' && gf[4] == 'x' && gf[5] == 'x'{
        return true;
    }
    if gf[6] == 'x' && gf[7] == 'x' && gf[8] == 'x'{
        return true;
    }
    if gf[0] == 'x' && gf[3] == 'x' && gf[6] == 'x'{
        return true;
    }
    if gf[1] == 'x' && gf[4] == 'x' && gf[7] == 'x'{
        return true;
    }
    if gf[0] == 'x' && gf[4] == 'x' && gf[8] == 'x'{
        return true;
    }
    if gf[2] == 'x' && gf[4] == 'x' && gf[6] == 'x'{
        return true;
    }
    if gf[2] == 'x' && gf[5] == 'x' && gf[8] == 'x'{
        return true;
    }

    //check to o win
    if gf[0] == 'o' && gf[1] == 'o' && gf[2] == 'o'{
        return true;
    }
    if gf[3] == 'o' && gf[4] == 'o' && gf[5] == 'o'{
        return true;
    }
    if gf[6] == 'o' && gf[7] == 'o' && gf[8] == 'o'{
        return true;
    }
    if gf[0] == 'o' && gf[3] == 'o' && gf[6] == 'o'{
        return true;
    }
    if gf[1] == 'o' && gf[4] == 'o' && gf[7] == 'o'{
        return true;
    }
    if gf[0] == 'o' && gf[4] == 'o' && gf[8] == 'o'{
        return true;
    }
    if gf[2] == 'o' && gf[4] == 'o' && gf[6] == 'o'{
        return true;
    }
    if gf[2] == 'o' && gf[5] == 'o' && gf[8] == 'o'{
        return true;
    }

    return false;
}

pub fn is_game_field_filled(gf : &Vec<char>)->bool{
    for i in 0..gf.len(){
        if gf[i] == ' '{
            return false;
        }
    }
    return true;
}

