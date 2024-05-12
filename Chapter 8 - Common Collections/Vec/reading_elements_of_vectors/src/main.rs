/*
 Ci sono due modi di riferirsi ai valori memorizzati in un vettore.
 Usando get o via indice.

 Da notare che il modo più safe è utilizzando get poichè ci restituisce un Option<&T>
*/

fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
    

    // Rust fornisce queste due modalità di accesso per poterti dare una scelta
    // sul tipo di comportamento che deve assumere quando tenti di accedere in maniera
    // errata alla memoria
    // let does_not_exist = &v[100];
    // println!("{:?}", does_not_exist);
    // let does_not_exist = v.get(100);
    // println!("{:?}", does_not_exist);

    // problemi con il borrowing
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];

    // cannot borrow `v` as mutable because it is also borrowed as immutable
    // v.push(6);

    println!("The first element is: {first}");

    /*
     Perché il vettore dovrebbe fregarsene di quello che sta alla fine?
     questo ha a che fare con il funzionamento della classe vettore poiché aggiungere 
     un elemento alla fine potrebbe significare riallocare l'intero vettore copiando
     i vecchi elementi nel nuovo spazio.
     */

}
