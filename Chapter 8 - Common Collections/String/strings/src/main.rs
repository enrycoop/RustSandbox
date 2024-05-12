fn main() {
    // Diversi modi di istanziare una stringa
    let mut s = String::new();

    let data = "Initial contents";

    let s = data.to_string();

    let s = "Initial contents".to_string();

    let s = String::from("initial contents");

    // le stringhe sono codificate in UTF-8
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // può crescere in dimensione e il suo contenuto può cambiare come nei vec.
    // In più puoi usare l'operatore + o la macro format! per concatenare i valori delle stringhe
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {s1}");

    let mut s = String::from("lo");
    s.push('l');
    println!("s is {s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s1 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // Sebbene sembrerebbe che faccia tante copie, + muove l'ownership di s1

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;


    let s1 = String::from("01");
    let s2 = String::from("02");
    let s3 = String::from("2024");


    // la macro format! è più facile da leggere e usa i riferimenti quindi non prende
    // l'ownership di nessuno dei suoi parametri
    let s = format!("{s1}-{s2}-{s3}");
    println!("{}", format!("{s1}/{s2}/{s3}"));


    // Indici delle stringhe
    let mut s1 = String::from("Зд");
    // let h = &s1[0..1];
    // println!("{h}"); // questo codice va in panic!

    // poichè non si sa quanto occupa un singolo carattere lo possiamo convertire in char

    for c in s1.chars() {
        println!("{c}");
    }

    // come byte verranno stampati più byte rispetto ai char
    for c in s1.bytes() {
        println!("{c}");
    }
    if s1.contains("д") {
        println!("Owwww yeahhhh");
        s1 = s1.replace("д", "A");
        println!("{s1} replaced? yes");
    }


}
