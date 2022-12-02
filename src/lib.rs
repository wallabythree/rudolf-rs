mod db;
mod api;

pub struct Client {
    key: String,
    db: db::Session
}

impl Client {
    pub fn new(key: String) -> Self {
        Self { key, db: db::Session::new() }
    }

    pub fn get(&self, year: u16, day: u8) -> Result<String, String> {

        if let Ok(input) = self.db.get(&self.key, year, day) {
            return Ok(input);
        }

        println!("Fetching input from API...");

        let input = api::get(&self.key, year, day)
            .map_err(|err| err.to_string())?;

        self.db.save(&self.key, year, day, &input)
            .map_err(|err| err.to_string())?;
        
        Ok(input)
    }
}

