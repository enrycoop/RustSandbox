# Validando i Riferimenti con Lifetimes

I lifetime sono un altro tipo di generic che abbiamo già usato.
Piuttosto che assicurarci che un tipo ha il comportamento che vogliamo,
lifetimes assicurano che i riferimenti siano validi quanto ne abbiamo bisogno.

Ogni riferimento ha il suo _lifetime_, il quale corrisponde allo scope per la quale è valido. Il più delle volte lifetime sono impliciti ed inferiti cosi come i tipi. Dobbiamo annotare i tipi solo quando sono possibili più tipi nello stesso momento. Allo stesso modo, dobbiamo annotare lifetime quando le lifetime dei riferimenti potrebbero essere correlate in più modi differenti.
Rust richiede di annotare le relazioni utilizzando i parametri generici di lifetime per assicurarci che gli attuali riferimenti utilizzati a runtime saranno definitivamente validi.

## Prevenire Dangling References con Lifetimes

L'obiettivo pricipale dei lifetime è di prevenire i _dangling references_, i quali causano il fatto che un programma referenzi altri dati rispetto ai dati che si intende referenziare. Consideriamo il programma sottostante:

```Rust
fn main() {
    // non inizializzato quindi non valido
    let r;  

    {
        let x = 5;
        r = &x;
    }
    // tentiamo di usare un riferimento ad un valore che è fuori scope
    println!("r: {}", r);
}
```
Lo scope esterno dichiara una variabile chiamata ```r``` senza valore iniziale, e lo scope interno dichiara una variabile chiamata ```x``` con un valore iniziale di 5. Dentro lo scope interno, tentiamo di impostare il valore di ```r``` con il riferimento a ```x```. Quindi lo scope interno finisce e tenta di stampare il valore di ```r```. Questo codice non compilerà perché ciò a cui si riferisce r è fuori dal suo scope prima che possiamo utilizzarlo.
La variabile x non vive abbastanza. La ragione è che ```x``` è fuori dallo scope quando lo scope interno finisce, mentre, ```r``` sarà ancora valido.
Se Rust permettesse a questo codice di funzionare, ```r``` si riferirebbe a della memoria che è stata deallocata quando ```x``` è uscito fuori scope. 
Se Rust permettesse a questo codice di funzionare, ```r``` si riferirebbe a della memoria che \e stata deallocata quando ```x```
Quindi come fa Rust a determinare che il codice non è valido? usa un ```Borrow Checker```.

## Il Borrow Checker
Il ```borrow checker```  confronta gli scope per determinare se tutti i borrow (prestiti) sono validi.
Questo è il codice di prima con il check che effettua il BC.
```Rust
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```
Qui abbiamo annotato la lifetime di r con ```'a``` e la lifetime di x con ```'b```. 
A compile time, Rust confrontala dimensione delle due lifetime e vede che r ha lifetime ```'a``` ma che si riferisce ad una memoria con lifetime ```'b```. Il programma viene rifiutato perchè ```'b``` è più corto di ```'a```: il soggetto della reference non vive tanto a lungo quanto la reference.

Per fixare questo errore:
```Rust
fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+
```
Qui, x ha la lifetime di 'b, che in questo caso è maggiore di 'a. Questo significa che r può fare riferimento a x perché Rust sa che il riferimento in r sarà sempre valido mentre x è valido.

