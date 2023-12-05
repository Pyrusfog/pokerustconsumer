use std::io::stdin;
use serde_json;
use serde::{Deserialize,Serialize};

// #[derive(Debug,Serialize,Deserialize)]
// struct AbilitiesNameStatusPokemon {
//     name: String
// }

#[derive(Debug,Serialize,Deserialize)]
struct IndexAbilitiesStatusPokemon {
    name: String
}

#[derive(Debug,Serialize,Deserialize)]
struct AbilitiesStatusPokemon {
    ability: IndexAbilitiesStatusPokemon,
}

#[derive(Debug,Serialize,Deserialize)]
struct TypeNameStatusPokemon {
    name: String
}

#[derive(Debug,Serialize,Deserialize)]
struct TypeStatusPokemon {
    r#type: TypeNameStatusPokemon
}

#[derive(Debug,Serialize,Deserialize)]
struct NameStatusPokemon {
    name: String
}

#[derive(Debug,Serialize,Deserialize)]
struct StatusPokemon {
    base_stat: i32,
    stat: NameStatusPokemon,
    effort: i32
}


#[derive(Debug,Serialize,Deserialize)]
// #[serde(rename_all = "camelCase")]
struct JsonPara {
    // #[serde(rename = "camelCase")]
    abilities: Vec<AbilitiesStatusPokemon>,
    pub base_experience: i32,
    // forms: Vec<String>,
    // game_indices: Vec<String>,
    pub height: i32,
    // held_items: Vec<String>,
    pub id: i32,
    pub is_default: bool,
    pub location_area_encounters: String,
    // moves: Vec<String>,
    pub name: String,
    pub order: i32,
    // past_abilities: Vec<String>,
    // past_types: Vec<String>,
    // species: SpeciesPokemon,
    // sprites: Vec<String>,
    pub stats: Vec<StatusPokemon>,
    pub types: Vec<TypeStatusPokemon>,
    pub weight:i32,
}

fn getpoketypes(body: JsonPara)->JsonPara{
    for i in 0..body.types.len(){
        let mut type_poke = &body.types[i].r#type.name;
        if i < body.types.len(){
            type_poke = type_poke ;
        }
        println!("{}",type_poke);
    }
    body
}
fn getpokeinfo(poke_number: i32){
    let url = format!("https://pokeapi.co/api/v2/pokemon/{}",poke_number);
    let body:JsonPara = reqwest::blocking::get(url).unwrap().json().unwrap();
    // let typepokedata:JsonPara = body.clone();
    // getpoketypes(typepokedata);
    
    
    print!("#{} - {}                                ",poke_number, body.name.to_uppercase());
    // println!("{:?}",body.abilities[0].ability.name);
    // println!("=====================================================================");
    print!("Types: ");
    for i in 0..body.types.len(){
        print!("{}", body.types[i].r#type.name);
        if i < body.types.len()-1{
            print!(" | ")
        }
    }
    println!();
    println!("=====================================================================");
    print!("Weight: {} Kg | Height : {} m                ", body.weight,body.height);
    print!("Abilities: ");
    for i in 0..body.types.len(){
        print!("{}", body.abilities[i].ability.name);
        if i < body.types.len()-1{
            print!(" | ")
        }
    }
    println!("\n");
    println!("{} : {} ({})                                   {} : {} ({})",body.stats[0].stat.name.to_uppercase(),body.stats[0].base_stat,body.stats[0].effort,body.stats[5].stat.name.to_uppercase(),body.stats[5].base_stat,body.stats[5].effort);
    println!("{} : {} ({})                               {} : {} ({})",body.stats[1].stat.name.to_uppercase(),body.stats[1].base_stat,body.stats[1].effort,body.stats[3].stat.name.to_uppercase(),body.stats[3].base_stat,body.stats[3].effort);
    println!("{} : {} ({})                              {} : {} ({})",body.stats[2].stat.name.to_uppercase(),body.stats[2].base_stat,body.stats[2].effort,body.stats[4].stat.name.to_uppercase(),body.stats[4].base_stat,body.stats[4].effort);
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
    if poke_number < 1292{
        getpokeinfo(poke_number);
        // let body:JsonPara  = serde::de::Deserialize::deserialize(body).unwrap();
        // println!("{:#?}",body.stats[0].stat.name);
    }else {
        println!("Incorrect PokeNumber")
    }
    Ok(())
}
