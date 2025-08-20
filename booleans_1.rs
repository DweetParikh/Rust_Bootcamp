fn main(){
    let is_male: bool = true; 
    let is_abv_18: bool = false;

    if is_male {
        println!("You are a Male");
    } else {
        println!("You are not a Male");
    }

    if is_male && is_abv_18 {
        println!("You are a legal male");
    }
}
