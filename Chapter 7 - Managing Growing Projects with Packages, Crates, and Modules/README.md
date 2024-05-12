# GESTIRE GROSSI PROGETTI

1. [Module System](#module-system)
2. [Package e Crate](#package-e-crate)
3. [Cargo new walk through](#cargo-new-walk-through)
4. [Defining Modules to Control Scope and Privacy](#defining-modules-to-control-scope-and-privacy)
5. [Paths for Referring to an Item in the Module Tree](#paths-for-referring-to-an-item-in-the-module-tree)

## MODULE SYSTEM

---

- _Packages_: una feature di **Cargo** che permette di buildare, testare e condividere Crate.
- _Crates_: una alberatura di moduli che produce una libreria o un eseguibile.
- _Modules_ e _Use_: permettono di controllare l'organizzazione, scope e privacy dei paths.
- _Paths_: un modo di nominare un item, come ad esempio struct, funzioni e moduli.

## PACKAGE E CRATE

---

Un `Crate` è la più piccola quantità di codice che il compilatore considera per volta. Possono essere binari o librerie. Un binario produce un eseguibile e ha un main, mentre una libreria non hai un main e non compilano un eseguibile. Invece, definiscono funzionalità che intendono condividere su più progetti.

Il `Crate root` è un file sorgente da cui il Compilatore Rust comincia e produce un modulo root del tuo Crate.

Un `Package` è un bundle di 1 o più **Crates** che forniscono un set di funzionalità. Un package contiene un _Cargo.toml_ file che descrive come buildare questi Crates [Cargo è attualmente un package che contine il crate binario per il command-line tool]. Un package può contenere più crate binary ma al più una library crate.

### Cargo new walk through

Dopo che eseguiamo `cargo new`, vedremo che crea un progetto dove dentro c'è _Cargo.toml_ file che ci da un package. C'è anche una cartella _src_ che contiene il _main.rs_. Se apri il _Cargo.toml_ non c'è riferimento al _src/main.rs_. Cargo segue la convenzione che se src contiene un main.rs è una binary se contiene un lib.rs è una library. Un package può avere più binary crate mettendoli in una directory _src/bin_: ogni file sarà un crate binario separato.

## Defining Modules to Control Scope and Privacy

- ```use``` keyword: che porta un path nello scope
- ```pub``` keyword: rende gli item pubblici

### Module cheat Sheet

Qui forniamo una veloce guida di come moduli, path, use keyword e pub keyword lavorano nel compilatore e come molti sviluppatori organizzano il loro codice.

- **Start from the crate root**: quando compiliamo un crate, il compilatore prima guarda nella **crate root file** (_src/lib.rs_ per una libreria o _src/main.rs_ per un binario) in cerca del codice da compilare.
- **Declaring Modules**: nel **crate root file**, puoi dichiarare nuovi moduli; diciamo ad esempio che dichiari un modulo "garden" con ```mod garden;```. Il compilatore cercherà il codice del modulo in questi posti:
    1. Inline, nelle **curly brackets** che sostituiscono il ; seguente il ```mod garden```.
    2. Nel file _src/garden.rs_
    3. Nel file _src/garden/mod.rs_
- **Declaring Submodules**: in ogni file oltre la crate root, puoi dichiarare sottomoduli. Per esempio, tu puoi dichiarare ```mod vegetables;``` in _src/garden.rs_. Il compilatore cercherà il codice dei sottomoduli nelle directory chiamate come il parent module in questi posti:
    1. Inline, direttamente seguente ```mod vegetables```, nelle curly brackets al posto del ;.
    2. Nel file _src/garden/vegetables.rs_
    3. Nel file _src/garden/vegetables/mod.rs_
- **Paths to code in modules**: una volta che il modula è parte del tuo crate, puoi riferirti al codice in quel modulo da qualsiasi parte nello stesso crate, fintanto che le privacy rule lo permettono, usando il path al codice. Per esempio, un tipo ```Asparagus``` nel modulo giardino vegetali si troverà in ```crate:garden:vegetables::Asparagus```.
- **Private vs public**: il codice in un modula è privato dai suoi parent modules per default. Per renderlo public, dichiaralo ```pub mod``` anzichè ```mod```. Per rendere gli elementi all'interno pubblici, utilizzare ```pub``` prima delle dichiarazioni.
- **The ```use``` keyword**: in uno scope, la ```use``` keyword crea uno shortcut agli elementi per ridurre la ripetizioni di path lunghi. In ogni scope puoii creare uno shortcut ```use crate:garden:vegetables::Asparagus;``` e da li hai bisogno di scrivere solo ```Asparagus``` per utilizzarlo nello scope.

Quindi creamo un binary crate chiamato ```backyard``` e illustriamo queste regole. La crate directory, contiene questa alberatura.
```
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

### Grouping Related Code in Modules
I moduli ci consentono di organizzare il codice e la privacy degli elementi, perchè il codice nei moduli è privato di default. Gli elementi privati sono dettagli di implementazioni interne non disponibili al di fuori. Possiamo scegliere di rendere i moduli e gli elementi all'interno pubblici, il ché gli espone e permette a codice esterno di usarlo e di dipendere da loro.

Come esempio, scriviamo una libreria che fornisce le funzionalità di un ristorante. Nel gergo dei ristoranti, alcune parto sono definite come front of house (dove sono i clienti, ecc..) e altre come back of house (dove gli chef cucinano ecc..). 

Possiamo strutturare le funzioni in moduli innestati. Crea una nuova librerua chiamata **resturant** con il comando ```cargo new resturant --lib```.

```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

Da notare che l'intera alberatura è sotto il modulo implicito ```crate```.

## Paths for Referring to an Item in the Module Tree
Per mostrare a Rust dove trovare un elemento in un alberatura di moduli, utilizziamo un path allo stesso modo in cui usiamo i path per navigare il filesystem. 

Un path può avere due forme:
- Un **absolute path** è il pieno path partendo dalla crate root; per un codice di una crate esterna il path assoluto comincia dal crate name, per il corrente comincia con il letterale  ```crate```.
- Un **relative path** comincia dal modulo corrente e usa ```self```, ```superior```, o un identificatore del modulo corrente.

Entrambi, path assoluti e relativi, sono seguiti da uno o più identificativi separati da ```::```.

### Starting Relative Paths with _super_

Possiamo costruire path relativi che appartengono al modulo padre, utilizzando ```super``` all'inizio del path, in modo analogo a ```../``` del filesystem. 