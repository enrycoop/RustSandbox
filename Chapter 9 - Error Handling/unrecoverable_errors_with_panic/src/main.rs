/* UNRECOVERABLE ERRORS WITH PANIC!

a volte le cose brutte succedono nel tuo codice, e non c'è niente da fare.
In questi casi, rust ha la macro panic!. Ci sono due casi d'uso per panic:
intraprendere un azione che causa il panic nel nostro codice o tramite chiamata
esplicita della macro panic!.
In entrambi i casi , causiamo un panic nel nosto programma che di default stampano
un messaggio di fallimento, si sbroglia (propaga), pulisce lo stack ed esce.
Tramite una variabile d'ambiente puoi anche avere che Rust mostra la call stack 
quando un panic occorre.

*/

fn main() {
    println!("Hello, world!");
}
