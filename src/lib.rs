use std::fs;
use std::error::Error;

pub struct Config {
    pub html_head: String,
    pub rss_head: String,
    pub entries_folder: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next(); // drop the executable

        // First arg should be the file with the HTML header
        let mut html_head = match args.next() {
            Some(path) => path,
            None => return Err("Didn't get a HTML header file path"),
        };

        html_head = match fs::read_to_string(html_head) {
            Ok(contents) => contents,
            Err(_) => return Err("Failed to read HTML header file"),
        };

        let mut rss_head = match args.next() {
            Some(path) => path,
            None => return Err("Didn't get a RSS header file path"),
        };

        rss_head = match fs::read_to_string(rss_head) {
            Ok(contents) => contents,
            Err(_) => return Err("Failed to read RSS header file"),
        };

        let entries_folder = match args.next() {
            Some(path) => path,
            None => return Err("Didn't get an entries folder path"),
        };

        Ok(Config{
            html_head,
            rss_head,
            entries_folder,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let entries: Vec<_> = fs::read_dir(config.entries_folder)?
        .collect();
    println!("{:#?}", entries);

    Ok(())
}

#[cfg(test)]
mod tests {

}
