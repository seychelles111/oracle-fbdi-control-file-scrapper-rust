use reqwest::blocking::get;
use select::{document::Document, predicate::And, predicate::Class, predicate::Name, node::Node};
use std::io::Read;
use rayon::prelude::*;

const URLS: &[&str] = &[
    "https://docs.oracle.com/en/cloud/saas/supply-chain-management/23a/oefsc/importmeterreadings-3113.html",
];

fn control_files(node: &Node) -> bool {
    node.text().contains("Control files")
}

fn func_scap_doc(url: &str) -> Vec<String> {
    let resp = get(url).unwrap();
    let document = Document::from_read(resp).unwrap();

    let mut arr_ctl_total = Vec::new();

    for table_row in document.find(And(Name("tr"), Class("row"))) {
        let table_cells = table_row.find(Name("td"));
        let control_files_cell = table_cells.filter(control_files).next();

        if control_files_cell.is_some() {
            for anchor in table_row.find(Name("a")) {
                if let Some(href) = anchor.attr("href") {
                    arr_ctl_total.push(href.into());
                }
            }
        }
    }

    arr_ctl_total
}

fn main() {
    let everything: Vec<Vec<String>> = URLS.par_iter().map(|url| func_scap_doc(url)).collect();

    for inner_list in everything {
        for item in inner_list {
            println!("{}", item);
        }
    }
}