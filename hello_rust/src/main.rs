use std::io::stdin;
use serde_json;
use serde::{Deserialize,Serialize};

#[derive(Debug,Serialize,Deserialize)]
// #[serde(rename_all = "camelCase")]
struct JsonPara {
    // #[serde(rename = "camelCase")]
    // abilities: Vec<String>,
    base_experience: i32,
    // forms: Vec<String>,
    // game_indices: Vec<String>,
    height: i32,
    // held_items: Vec<String>,
    id: i32,
    is_default: bool,
    location_area_encounters: String,
    // moves: Vec<String>,
    name: String,
    order: i32,
    // past_abilities: Vec<String>,
    // past_types: Vec<String>,
    // species: Vec<String>,
    // sprites: Vec<String>,
    // stats: Vec<String>,
    // types: Vec<String>,
    weight:i32,
}
fn main() -> Result<(),reqwest::Error>{
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
    
    let url = format!("https://pokeapi.co/api/v2/pokemon/{}",poke_number);
    let body:JsonPara = reqwest::blocking::get(url).unwrap().json().unwrap();    
    // let body:JsonPara  = serde::de::Deserialize::deserialize(body).unwrap();
    println!("{:#?}",body);
    Ok(())

}
