// ORIGINAL
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// WITH LIFETIME
/* In pratica, significa che il lifetime della reference restituita da longest
è la stessa della lifetime di 'a.
Da ricordare che, quando specifichiamo il parametro lifetime non stiamo cambiando
la lifetime di ognuno dei falori passati o restituiti, piuttosto, stiamo specificando
al borrow checker che deve rifiutare ogni valore che non aderisce a questi vincoli.
Quando annotiamo la lifetime nelle funzioni, le annotazioni vanno nella signature,
non nel body. Le annotazioni lifetime diventano parte del contratto della funzione.
Avere nelle signature delle funzioni i contratti significa che l'analisi che il 
compilatore rust deve fare è semplice.
 */
fn longest<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}