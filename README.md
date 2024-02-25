# STPG (hackygit)
a small startpage for your (probably my) browser

## Installation
1. Clone the repository

## Usage
`<stpg> <spf_in> <html_out>`

## SPF Format
The SPF format is a simple format for defining a startpage. It is a text file with the following format:
```
*<category_name> <-- denotes the start of a new category
<url>|<text>|<description> <-- denotes a link
```
where `*` denotes the start of a new category, and `|` separates the link's URL from its text.

## Features 
- Some links
- Custom format for said links
- Good enough mobile experience (well, it's not terrible)
- Not much else right now