use std::fs;

use roxmltree::{Document, Node as XmlNode};

// these are pretty ugly otherwise ;)
type RowStats = (usize, Vec<usize>);
type TableStats = (usize, Vec<(usize, Vec<usize>)>);

const DOCUMENT: &str = "document.xml";

const NS: &str =
    "http://schemas.openxmlformats.org/wordprocessingml/2006/main";

// Match names in the namespace NS.
fn is_name(name: &'static str) -> impl FnMut(&XmlNode) -> bool {
    move |node| {
        let nn = node.tag_name();
        nn.name() == name && nn.namespace() == Some(NS)
    }
}

fn row_stats(node: XmlNode) -> Option<RowStats> {
    if !(is_name("tr")(&node)) {
        return None;
    }
    let cells: Vec<usize> = node
        .children()
        .filter(is_name("tc"))
        .map(|cell| cell.children().count())
        .collect();

    Some((cells.len(), cells))
}

fn main() -> Result<(), String> {
    let file = fs::read_to_string(DOCUMENT)
        .map_err(|err| format!("{}", err))?;

    let tables: Vec<TableStats> = Document::parse(&file)
        .map_err(|err| format!("{}", err))?
        .descendants()
        .filter(is_name("tbl"))
        .map(|tbl| {
            let rows: Vec<RowStats> =
                tbl.children().filter_map(row_stats).collect();

            (rows.len(), rows)
        })
        .collect();

    println!("{:?}", tables);

    let expected: Vec<TableStats> = vec![
        (2, vec![(4, vec![2, 2, 2, 2]), (4, vec![2, 2, 2, 2])]),
        (2, vec![(4, vec![2, 2, 2, 2]), (4, vec![2, 2, 2, 2])]),
    ];
    assert_eq!(tables, expected);
    Ok(())
}
