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

fn main() -> Result<(), String> {
    let file = fs::read_to_string(DOCUMENT)
        .map_err(|err| format!("{}", err))?;

    let doc = roxmltree::Document::parse(&file)
        .map_err(|err| format!("{}", err))?;

    let tbls = doc
        .descendants()
        .filter(is_name("tbl"))
        .collect::<Vec<roxmltree::Node>>();

    let tbls: Vec<(usize, Vec<(usize, Vec<usize>)>)> = tbls
        .iter()
        .map(|tbl| {
            let rows = tbl
                .children()
                .filter(is_name("tr"))
                .map(|row| {
                    let cells = row
                        .children()
                        .filter(is_name("tc"))
                        .map(|n| {
                            n
                                .children()
                                .collect::<Vec<roxmltree::Node>>()
                                .len()
                        })
                        .collect::<Vec<usize>>();
                    (cells.len(), cells)
                })
                .collect::<Vec<(usize, Vec<usize>)>>();
            (rows.len(), rows)
        }).collect();

    println!("{:?}", tbls);
    Ok(())
}
