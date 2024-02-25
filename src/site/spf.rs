// spf is the StPg Format, a poorly designed text format for storing links with names inside a file, with categories.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::site::{Category, Link};

pub fn from_file(path: &Path) -> Option<Vec<Category>> {
    // get a handle to the file
    let file = File::open(path).ok()?;
    // create a buffered reader
    let reader = BufReader::new(file);
    // create a vector to store the categories we find
    let mut categories = Vec::new();
    // create a variable to store the current category we are reading
    let mut current_category = None;
    for line in reader.lines() {
        let line = line.ok()?;
        // if the line starts a new category, add the current category to the list and start a new one
        if line.starts_with('*') {
            if let Some(category) = current_category {
                // add the current category to the list
                categories.push(category);
            }
            // start the new category
            current_category = Some(Category {
                name: line.strip_prefix('*')?.trim().to_string(),
                links: Vec::new(),
            });
        } else if let Some(ref mut category) = current_category {
            // if we are in a category, add the link to the category
            let mut parts = line.split('|');
            // if the line is not in the correct format, skip it
            let url = parts.next()?.to_string();
            let text = parts.next()?.to_string();

            category.links.push(Link { url, text });
        }
    }
    if let Some(category) = current_category {
        // add the last category to the list
        categories.push(category);
    }
    Some(categories)
}
