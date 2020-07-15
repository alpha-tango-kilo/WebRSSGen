extern crate chrono;
use chrono::offset::Utc;
use chrono::DateTime;

use std::fs;
use std::error::Error;

pub struct Config {
    pub html_head: String,
    pub rss_head: String,
    pub entries_folder: String,
}

impl Config {
    // TODO: rewrite with proper errors
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

        // Second arg should be the file with the RSS header
        let mut rss_head = match args.next() {
            Some(path) => path,
            None => return Err("Didn't get a RSS header file path"),
        };

        rss_head = match fs::read_to_string(rss_head) {
            Ok(contents) => contents,
            Err(_) => return Err("Failed to read RSS header file"),
        };

        // Third arg should be the path that contains all the HTML entries
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

#[derive(Debug)]
struct Entry {
    title: String,
    date: String,
    body: String,
}

impl Entry {
    fn new(path: String) -> Result<Entry, Box<dyn Error>> {
        let body = fs::read_to_string(&path)?;

        // Path is already checked by this point so an unwrap should be safe
        let date: DateTime<Utc> = fs::metadata(&path)
            .unwrap()
            .modified()? // probably should handle better, default String maybe?
            .into();
        // Format: "2001-01-25 23:07"
        let date = date.format("%F %R").to_string();

        // Get file name by splitting on slashes and taking the last part
        // or just taking the whole path if there are no slashes
        let title = path.rsplit(|c| c == '/' || c == '\\')
            .next()
            .unwrap();
        let title = if title.ends_with(".xml") {
            &title[..title.len()-4]
        } else {
            &title[..title.len()-5]
        }.to_string();

        Ok(Entry {
            title,
            date,
            body,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read file list and generate entries from it
    let entries = fs::read_dir(config.entries_folder)?
        // Get out only the files we have permission to read
        .filter_map(|rde| {
            // Convert to Option<DirEntry>
            rde.ok()
                // On the Some()s, run this
                .and_then(|de| -> Option<String> {
                    // Get path
                    let pb = de.path();
                    // Check the extension
                    match pb.extension() {
                        // If there is an extension...
                        Some(ext) => if ext == "xml" || ext == "html" {
                            // ... and it's XML or HTML... (is the above case sensitive?)
                            pb.into_os_string()
                                .into_string()
                                .ok() // ... convert it out to an Option<String>
                        } else {
                            // If it's not the correct extension, reject
                            None
                        },
                        // If no extension, reject
                        None => None,
                    }
                })
        })
        // Shove all those relevant paths through Entry::new...
        .map(Entry::new)
        // ...and get out a load of entries, returning an error if any one of them failed
        .collect::<Result<Vec<_>, _>>()?;
    println!("{:#?}", entries);

    Ok(())
}

#[cfg(test)]
mod tests {

}
