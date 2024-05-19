/* Un Tratto (o Trait) definisce le funzionalità che un particolare tipo ha 
e può condividere con altri tipi. Possiamo usare i trait bounds per specificare
un generico tipo può essere un tipo che ha un certo comportamento. (tipo le interfacce)*/


// lo abbiamo dichiarato pub così i crate che dipendono da questo crate possono usarlo
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
