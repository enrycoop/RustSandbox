# Splitting a program when main starts getting large
The Rust community has developed guidelines for splitting the separate concerns
of a binary program when main starts getting large. This process has the
following steps:
  1) Split your program into a main.rs and a lib.rs and move your program’s logic
    to lib.rs.
  2) As long as your command line parsing logic is small, it can remain in main.rs.
  3) When the command line parsing logic starts getting complicated, extract it
    from main.rs and move it to lib.rs.

The responsibilities that remain in the main function after this process should
be limited to the following:
  1) Calling the command line parsing logic with the argument values
  2) Setting up any other configuration
  3) Calling a run function in lib.rs
  4) Handling the error if run returns an error

## Test Driven Development (TDD)
Si compone dei seguenti step:
1) scrivi un test che fallisce ed eseguilo per assicurarti che fallisca per le ragioni che ti aspetti
2) Scrivi o modifica solo il codice che permette ai nuovi test di passare.
3) Rifattorizza il codice appena aggiunto o cambiato e assicurati che i test continuino a passare.
4) ripeti dal passo 1!

Questa e` solo una delle tante modalità di scrivere il codice, TDD puo aiutare durante la fase di design. Scrivere i test prima di scrivere codice aiuta ad avere una grande test coverage durante il processo.
