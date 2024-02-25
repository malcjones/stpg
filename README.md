# STPG (hackygit)
a small startpage for your (probably my) browser

## Installation
1. Clone the repository

## Usage
`<stpg> <spf_in> <html_out>`

## SPF Format
The SPF format is a simple format for defining a startpage. It is a text file with the following format:
```
*<category1_name>
<category1_link1_url>|<category1_link1_text>
<category1_link2_url>|<category1_link2_text>
*<category2_name>
<category2_link1_url>|<category2_link1_text>
<category2_link2_url>|<category2_link2_text>
```
where `*` denotes the start of a new category, and `|` separates the link's URL from its text.

## Features 
- Some links
- Custom format for said links
- Good enough mobile experience (well, it's not terrible)
- Not much else right now