mod site;

use site::start_page;

fn main() {
    let input_name = std::env::args().nth(1);
    let output_name = std::env::args().nth(2);
    if input_name.is_none() || output_name.is_none() {
        println!("usage: stpg <input> <output>");
        return;
    }
    let input_name = input_name.unwrap();
    let output_name = output_name.unwrap();

    let categories = site::spf::from_file(std::path::Path::new(&input_name));
    if categories.is_none() {
        println!("could not read file");
        return;
    }
    
    let page = render_page(categories.unwrap());
    std::fs::write(output_name.clone(), page).expect("could not write file");

    println!("wrote page to {}", output_name);
}

fn render_page(categories: Vec<site::Category>) -> String {
    println!(
        "making page (this should only happen once) from {}",
        std::env::args().nth(1).unwrap()
    );
    println!("categories: {}", categories.len());
    println!("{}", category_tree(categories.clone()));
    start_page(categories).into_string()
}

fn category_tree(category: Vec<site::Category>) -> String {
    // render a unicode tree of the categories
    let mut tree = String::new();
    for cat in category {
        tree.push_str(&format!(
            "{} {}\n",
            site::category_icon(&cat.name),
            cat.name
        ));
        for link in cat.links {
            tree.push_str(&format!("  - {}\n", link.text));
        }
    }
    tree
}
