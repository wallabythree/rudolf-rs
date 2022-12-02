use rusqlite::{Connection, Result};

pub struct Session {
    conn: rusqlite::Connection
}

impl Session {
    pub fn new() -> Self {
        Self {  conn: Connection::open("./rudolf.db3").unwrap() }
    }

    pub fn get(
        &self,
        session: &str,
        year: u16,
        day: u8
    ) -> Result<String, rusqlite::Error> {

        let input: Result<String> = self.conn.query_row(
            "SELECT input
            FROM puzzles
            WHERE session = ? AND year = ? AND day = ?",
            rusqlite::params![session, year, day],
            |row| row.get(0)
        );

        input
    }

    pub fn save(
        &self,
        session: &str,
        year:u16,
        day: u8,
        input:&str
    ) -> Result<(), rusqlite::Error> {

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS puzzles (
                session TEXT NOT NULL,
                year INTEGER NOT NULL,
                day INTEGER NOT NULL,
                input TEXT NOT NULL,
                PRIMARY KEY (session, year, day))
            ",
            ()
        )?;

        self.conn.execute(
            "INSERT INTO puzzles VALUES (?1, ?2, ?3, ?4)",
            (session, year, day, input)
        )?;

        Ok(())
    }
}

