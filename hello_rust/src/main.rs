use std::io::stdin;
fn main() {
    let mut poke_number = String::new();
    println!("Enter a number between 1 and 1292");
    println!("Poke Number: ");
    match stdin().read_line(&mut poke_number){
        Ok(_siz) => {},
        Err(_err) => println!("Nao deu certo")
    };
    
    let poke_number: i32 = match poke_number.trim().parse(){
        Ok(poke_number) => poke_number,
        Err(_err) => 0i32
    };
    
    if poke_number > 1292{
        println!("Nao funcionou, Nao tem pokemons com esse numero");
    }else {
        let url = format!("https://pokeapi.co/api/v2/pokemon/{}",poke_number);
        let body = match reqwest::blocking::get(url){
          Ok(r) => r,
          Err(_err) => {
            println!("Request failed: {}", _err.to_string());
            return;
          }  
        };
        println!("{:?}",body.text())
    }

}
