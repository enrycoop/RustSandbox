# Common Collections
La standard library di Rust include un numero utile di strutture dati chiamate **collections**. Molti altri tipi di dato rapresentano un valore specifico, ma le collection possono contenere più valori. A differenza degli array built-in e delle tuple, i dati alla quale queste collection puntano sono memorizzate nello heap, il ché significa che la quantità di dati non deve essere conosciuta a compile time e può crescere o restringersi durante la run.

Ogni tipo di collection ha differenti capacità e costi. Discuteremo tre tipi di collection utilizzate molto spesso nei programmi Rust:
- **Vector**: ti permette di memorizzare un numero variabile di valori sequenzialmente.
- **String**: è una collezione di caratteri. Abbiamo già menzionato il tipo ```string```, ma in questo capitolo ne parleremo più nel dettaglio.
- **Hash Map**: permettono di creare coppie chiave-valore. È una particolare implementazione della struttura più generale _map_.

Per imparare di più sugli altri tipi di collections si può dare un'occhiata alla standard library.