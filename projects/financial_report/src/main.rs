use postgres::{Client, Error, NoTls};
use chrono::NaiveDate;
use bigdecimal::BigDecimal;

#[derive(Debug)]
struct Transaction {
    date: String,
    category: String,
    from: String,
    to: String,
    value: BigDecimal,
    description: String,
    typology: String,
}

impl Transaction {
  pub fn build (date: NaiveDate, category: String, from: String, to: String, value: String, description: String, typology: String) -> Self {
    Transaction {
      date: date.format("%Y-%m-%d").to_string(),
      category: category.trim().to_string(),
      from: from.trim().to_string(),
      to: to.trim().to_string(),
      value: BigDecimal::from(&value).unwrap(),
      description: description.trim().to_string(),
      typology: typology.trim().to_string(),
  }
  }
}

fn main() -> Result<(), Error> {
  let mut client = Client::connect("postgresql://postgres:postgres@localhost/financial", NoTls)?;
  
  // Verifica connessione
  if client.is_closed() {
    println!("Errore: Connessione chiusa");
    return Ok(());
  }

  for row in client.query("SELECT * FROM transactions", &[])? {
    let date: NaiveDate = row.get("date");
    let category: String = row.get("category");
    let from: String = row.get("from");
    let to:String = row.get("to");
    let value:String = row.get("value");
    let description:String = row.get("description");
    let typology:String = row.get("type");

    let transaction = Transaction::build(date, category, from, to, value, description, typology);
    println!("{:?}", transaction);
  }
  Ok(())
}
