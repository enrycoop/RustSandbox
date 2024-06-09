/*
Le closure di rust sono funzioni anonime che puoi salvare in
una variabile o passare come argomento ad altre funzioni.
Puoi creare le closure in un posto e poi chiamarle da qualche
altra parte. per valutarle in un contesto diverso.
Diversamente dalle funzioni, le closure possono catturare
valori dallo scope dove esse sono definite.
 */


/*
SCENARIO:
ogni tanto, la nostra compagnia di t-shirt da una t-shirt esclusiva,
edizione limitata a qualcuno nella mailing list. Le persone
nella mailing list possono opzionalmente aggiungere il loro colore
preferito sul loro profilo.
Se la persona scelta ha il suo colore preferito settato, ottengono
quel colore di t-shirt, altrimenti un colore a caso.
 */

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}


fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "the user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
