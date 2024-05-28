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

In definitiva, la sintassi della durata riguarda la connessione delle durate di vari 
parametri e dei valori restituiti delle funzioni. Una volta che sono collegati, 
Rust ha abbastanza informazioni per consentire operazioni sicure per la memoria e 
non consentire operazioni che creerebbero puntatori penzolanti o violerebbero in altro modo 
la sicurezza della memoria.
 */
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}
