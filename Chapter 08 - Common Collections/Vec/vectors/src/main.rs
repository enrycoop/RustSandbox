fn main() {
    // Vec<T> bisogna specificare il tipo poiché Vec è implementato usando le generics
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);

    // per inizializzare il vettore si può usare la macro vec!
    let v = vec![1, 2, 3];

    println!("{:?}", v);
    
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("{:?}", v);

    v.pop();

    println!("{:?}", v);

    v.remove(1);

    println!("{:?}", v);
}