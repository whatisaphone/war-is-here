use glob::glob;
use grep::{
    matcher::{Captures, Matcher},
    regex::RegexMatcher,
    searcher::{sinks::UTF8, SearcherBuilder},
};
use indexmap::IndexMap;
use std::{
    env,
    fmt::{self, Write},
    fs,
    path::PathBuf,
};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let mut searcher = SearcherBuilder::new().multi_line(true).build();
    let matcher = RegexMatcher::new(r"struct_wrapper_super![(]\s*(.+?),\s*(.+?),?\s*\);").unwrap();
    let mut captures = matcher.new_captures().unwrap();

    let mut extends = IndexMap::new();

    for path in glob(manifest_dir.join("src/**/*.rs").to_str().unwrap()).unwrap() {
        searcher
            .search_path(
                &matcher,
                path.unwrap(),
                UTF8(|_line_number, matsh| {
                    let matches = matcher.captures(matsh.as_bytes(), &mut captures).unwrap();
                    assert!(matches);
                    let subclass = matsh[captures.get(1).unwrap()].to_string();
                    let superclass = matsh[captures.get(2).unwrap()].to_string();
                    // let cast_method = &matsh[captures.get(3).unwrap()].to_string();
                    extends
                        .entry(superclass)
                        .or_insert_with(Vec::new)
                        .push(subclass);
                    Ok(true)
                }),
            )
            .unwrap();
    }

    let mut generated = String::with_capacity(extends.len() * 100);

    for (superclass, subclasses) in &extends {
        emit_impls(&mut generated, &extends, superclass, subclasses).unwrap();
    }

    fs::write(&out_dir.join("cast.rs"), &generated).unwrap();
}

fn emit_impls(
    w: &mut impl Write,
    extends: &IndexMap<String, Vec<String>>,
    superclass: &str,
    subclasses: &[String],
) -> fmt::Result {
    for subclass in subclasses {
        writeln!(
            w,
            "
impl AsRef<{superclass}> for gfc::{subclass} {{
    fn as_ref(&self) -> &{superclass} {{ self }}
}}",
            superclass = superclass,
            subclass = subclass,
        )?;

        if let Some(subsubclasses) = extends.get(&format!("gfc::{}", subclass)) {
            emit_impls(w, extends, superclass, subsubclasses)?;
        }
    }

    Ok(())
}
