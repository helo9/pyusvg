use pyo3::prelude::*;
use usvg;
use usvg::{TreeParsing, TreeWriting, TreeTextToPath};

#[pyfunction]
fn simplify(text: &str) -> Option<String> {
    let mut fontdb = usvg_text_layout::fontdb::Database::new();

    fontdb.load_system_fonts();

    fontdb.set_serif_family("Times New Roman");
    fontdb.set_sans_serif_family("Arial");
    fontdb.set_cursive_family("Comic Sans MS");
    fontdb.set_fantasy_family("Impact");
    fontdb.set_monospace_family("Courier New");

    return match usvg::Tree::from_str(text, &usvg::Options::default()) {
        Ok(mut tree) => {
            tree.convert_text(&fontdb);
            Some(tree.to_string(&usvg::XmlOptions::default()))
        },
        Err(e) => {
            println!("Got parsing error: {}", e);
            None
        }
    }
}

#[pymodule]
fn pyusvg(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(simplify, m)?)?;

    Ok(())
}
