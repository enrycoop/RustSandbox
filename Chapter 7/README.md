# GESTIRE GROSSI PROGETTI

1. [Module System](#module-system)
2. [Package e Crate](#package-e-crate)

## MODULE SYSTEM

---

- _Packages_: una feature di **Cargo** che permette di buildare, testare e condividere Crate.
- _Crates_: una alberatura di moduli che produce una libreria o un eseguibile.
- _Modules_ e _Use_: permettono di controllare l'organizzazione, scope e privacy dei paths.
- _Paths_: un modo di nominare un item, come ad esempio struct, funzioni e moduli.

## PACKAGE E CRATE

---

Un `Crate` è la più piccola quantità di codice che il compilatore considera per volta. Possono essere binari o librerie. Un binario produce un eseguibile e ha un main, mentre una libreria non hai un main.

Un `Package` è un bundle di 1 o più **Crates** che forniscono un set di funzionalità.
