use gluesql::prelude::*;

fn main() {
    let storage = SledStorage::new("data/doc-db").unwrap();
    let mut glue = Glue::new(storage);
    let sqls = vec![
        "DROP TABLE IF EXISTS Glue;",
        "CREATE TABLE Glue (id INTEGER, name TEXT);",
        "INSERT INTO Glue VALUES (1, '최지석'), (2, '문태훈'), (3, '위성훈'), (4, '조형관');",
        "SELECT name FROM Glue WHERE id in (2,4);",
    ];

    for sql in sqls {
        let output = glue.execute(sql).unwrap();
        println!("{:?}", output)
    }
}
