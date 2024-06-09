/*
Una volta che una chiusura ha catturato un riferimento o ha preso possesso
di un valore dall’ambiente in cui è definita la chiusura (influenzando così
ciò che, se presente, viene spostato all’interno della chiusura), il codice
nel corpo della chiusura definisce cosa succede ai riferimenti o ai valori
quando la chiusura viene valutata in seguito (influenzando così ciò che, se
presente, viene spostato fuori dalla chiusura). Il corpo di una chiusura può
fare una delle seguenti azioni: 
 - spostare un valore catturato fuori dalla chiusura, 
 - mutare il valore catturato, 
 - né spostare né mutare il valore,
 - non catturare nulla dall’ambiente iniziale.

Il modo in cui una chiusura cattura e gestisce i valori dall’ambiente
influisce su quali tratti implementa la chiusura, e i tratti sono il modo in
cui le funzioni e le strutture possono specificare quali tipi di chiusure 
possono utilizzare. Le chiusure implementeranno automaticamente uno, due o 
tutti e tre questi tratti Fn, in modo additivo, a seconda di come il corpo 
della chiusura gestisce i valori:

 - FnOnce si applica alle chiusure che possono essere chiamate una volta.
  Tutte le chiusure implementano almeno questo tratto, perché tutte le 
  chiusure possono essere chiamate. Una chiusura che sposta i valori 
  catturati fuori dal suo corpo implementerà solo FnOnce e nessuno degli 
  altri tratti Fn, perché può essere chiamata solo una volta. 
 - FnMut si applica alle chiusure che non spostano i valori catturati fuori 
 dal loro corpo, ma che potrebbero mutare i valori catturati. Queste 
 chiusure possono essere chiamate più di una volta. 
 - Fn si applica alle chiusure che non spostano i valori catturati fuori dal 
 loro corpo e che non mutano i valori catturati, così come le chiusure che 
 non catturano nulla dal loro ambiente. Queste chiusure possono essere 
 chiamate più di una volta senza mutare il loro ambiente, il che è 
 importante in casi come chiamare una chiusura più volte contemporaneamente.

*/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);

    /*
    let mut sort_operations = vec![];
    let value = String::from("by key called");

    list.sort_by_key(|r| {
        sort_operations.push(value);
        r.width
    });
    println!("{:#?}", list);
    */
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}
