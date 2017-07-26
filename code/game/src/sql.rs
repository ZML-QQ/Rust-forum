use postgres::{Connection, SslMode};
use mode::Date;

pub fn start() -> (usize,Vec<String>) {
    let conn = Connection::connect("postgres://admin:admin@localhost/mydb", SslMode::None).unwrap();
    // conn.execute("CREATE TABLE world (
    //                id              SERIAL PRIMARY KEY,
    //                name            VARCHAR NOT NULL,
    //                data            BYTEA
    //              )", &[]).unwrap();
    // let he = Date {
    //                   id: 0,
    //                   name: "Actions speak louder than words".to_owned(),
    //                   data: None
    //               };
    //  conn.execute("INSERT INTO world (name, data) VALUES ($1, $2)",
    //                            &[&he.name, &he.data]).unwrap();

    let mut thelines: Vec<String> = Vec::new();
    for row in &conn.query("SELECT id, name, data FROM world", &[]).unwrap() {
         let line = Date {
             id: row.get(0),
             name: row.get(1),
             data: row.get(2)
         };
     println!("Found line {}", line.name);
     thelines.push(line.name);
    }
    (thelines.len(),thelines)
}
