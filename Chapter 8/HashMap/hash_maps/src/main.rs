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
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // Iteriamo
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
