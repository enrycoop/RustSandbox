/*
Il pattern itarator permette di eseguire dei task su una sequenza di elementi
a turno. Un iteratore è responsabile della logica di iterazione su ogni
elemento e determinare quando la sequenza è finita. 
Gli iteratori in rust sono lazy, cioè non hanno effetto finchè non chiami i 
metodi che consumano l'iteratore.
 */

// tutti gli iterator implementano un trait chiamato Iterator definito nella
// standard library nel seguente modo:
pub trait Iterator {

    type Item; // tipo associato al trait.

    fn next(&mut self) -> Option<Self::Item>;
}
// implementare l'Iterator trait richiede anche di definire il tipo Item

fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        /*
        Da notare che i valori che otteniamo dalle chiamate a next sono
        riferimenti immutabili ai valore del vettore. Se lo vogliamo di
        valori owned dobbiamo chiamare into_iter, similmente se vogliamo 
        riferimenti mutabili iter_mut.
         */
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }

}