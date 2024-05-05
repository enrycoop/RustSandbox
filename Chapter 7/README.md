# GESTIRE GROSSI PROGETTI

1. [Module System](#module-system)
2. [Package e Crate](#package-e-crate)
3. [Cargo-New walk through](#cargo-new-walk-trhough)

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
