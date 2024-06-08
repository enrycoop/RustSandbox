/*
  L'ultimo tipo di cui parliamo è HashMap<K, V>, memorizza chiavi valore usando la funzione
  di hashing che determina come mettere questi chiave valore in memoria.
  Sono utili quando vuoi cercare i dati non per indice ma usando una chiave di un qualsiasi
  tipo.

*/

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("orange"), 50);

    // Accedere agli elementi in un Hash Map
    let team_name = String::from("Blue");

    // usiamo copied per ottenere un Option<i32> invece di un Option<&i32>
    let _score = scores.get(&team_name).copied().unwrap_or(0);

    // Iteriamo
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Ownership
    let field_name = String::from("Favourite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, &field_value);
    // field_name e field_value non sono più valide poichè è stata fatta la move
    //println!("{}", field_name);

    // UPDATING A VALUE

    // overwriting
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // adding if a key isn't present
    scores.entry(String::from("orange")).or_insert(50);
    scores.entry(String::from("orange")).or_insert(50);
    
    println!("{:?}", scores);

    // updating a value based on the old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    
    for word in text.split_whitespace() {
      let count = map.entry(word).or_insert(0);
      *count += 1;
    } // count è fuori scope quindi è safe

    println!("{:?}", map);

}
