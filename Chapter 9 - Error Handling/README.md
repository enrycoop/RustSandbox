# UNRECOVERABLE ERRORS WITH panic!

A volte le cose brutte succedono nel tuo codice, e non c'è niente da fare.
In questi casi, rust ha la macro ```panic!```. 
Ci sono due casi d'uso per panic:
- intraprendere un azione che causa il panic nel nostro codice 
-  tramite chiamata
esplicita della macro panic!.

In entrambi i casi , causiamo un panic nel nosto programma che di default stampano
un messaggio di fallimento, si sbroglia (propaga), pulisce lo stack ed esce.
Tramite una variabile d'ambiente puoi anche avere che Rust mostra la call stack 
quando un panic occorre.

## Propagazione dello Stack o Aborto in Risposta di un Panic

Per default, quando un panic occorre, il programma comincia a Sbrogliarsi, il ché significa che Rust torna indietro nello stack e pulisce i dati da ogni funzione che incontra. Comunque, questo tornare indietro e pulire è molto lavoro. Perciò Rust ti permette di scegliere l'alternativa di abortire immediatamente, il chè fa terminare il programma senza pulizia.
La memoria che il programma stava utilizzando deve essere pulita dal sistema operativo. Se nel tuo progetto hai bisogno che il binario risultante sia il più piccolo possibile, puoi switchare da Sbrogliare ad bortire il **panic** aggiungendo ```panic = 'abort'``` alla sezione ```[profile]``` appropriato del tuo file ```Cargo.toml```.
Per esempio, se vuoi abortire al panic in release mode aggiungi:
```
[profile.release]
panic = 'abort'
```

# RECOVERABLE ERRORS WITH Result

Molti errori non sono così seri da richiedere che il programma si fermi del tutto.
A volte, quando una funzione fallisce, è per una ragione che tu puoi facilmente interpretare e rispondere. Per esempio se cerchi di aprire un file e l'operazione fallisce perchè non esiste, potresti voler creare il file invece di terminare il processo.
Richiamando dal capitolo 2 il ```Result``` enum (diverso dal ```Optional```) è definito come avere due varianti ```Ok``` ed ```Err```
```Rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```