use roxmltree;

use std::fs;

const DOCUMENT: &str = "document.xml";

const NS: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";

// Match names in the namespace NS.
fn is_name(name: &'static str) -> impl FnMut(&roxmltree::Node) -> bool {
    move |node| {
        let nn = node.tag_name();
        nn.name() == name && nn.namespace() == Some(NS)
    }
}

fn process_row(node: roxmltree::Node) -> Option<(usize, Vec<usize>)> {
    if !(is_name("tr")(&node)) {
        return None;
    }
    let cells: Vec<usize> = node
        .children()
        .filter(is_name("tc"))
        .map(|cell| cell.children().collect::<Vec<_>>().len())
        .collect();

    Some((cells.len(), cells))
}

fn main() -> Result<(), String> {
    let file = fs::read_to_string(DOCUMENT)
        .map_err(|err| format!("{}", err))?;

    let doc = roxmltree::Document::parse(&file)
        .map_err(|err| format!("{}", err))?;

    let tbls: Vec<(usize, Vec<(usize, Vec<usize>)>)> = doc
        .descendants()
        .filter(is_name("tbl"))
        .map(|tbl| {
            let rows = tbl
                .children()
                .filter_map(process_row)
                .collect::<Vec<(usize, Vec<usize>)>>();

            (rows.len(), rows)
        }).collect();

    println!("{:?}", tbls);
    Ok(())
}
