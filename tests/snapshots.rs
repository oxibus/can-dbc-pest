#![cfg(feature = "encoding")]

use std::fs;
use std::path::Path;

use can_dbc_pest::{decode_cp1252, DbcParser, Rule};
use insta::{assert_debug_snapshot, with_settings};
use pest::Parser;

/// Test parsing all DBC files
#[test]
fn test_cantools_dbc_files() {
    let test_dirs = [("tests/shared-test-files/dbc-cantools", "dbc-cantools")];
    for (path, file_name) in &test_dirs {
        let path = Path::new(path);
        let snapshot_path = String::from("snapshots-") + file_name;
        // snapshots should go to /tests/snapshots-* directory
        with_settings! {
            { omit_expression => true,
              snapshot_path => snapshot_path ,
              prepend_module_to_snapshot => false },
            {
                test_dbc_files(path);
            }
        }
    }
}

/// Test parsing all DBC files in the given directory.
fn test_dbc_files(dir: impl AsRef<Path>) {
    let dir = dir.as_ref();
    let dir_display = dir.display();
    let dir_content = fs::read_dir(dir).unwrap_or_else(|e| {
        panic!(
            "
--------------------------------------------------------------------------
Error reading dbc test files from   {dir_display}
{e}
Make sure git submodules are up to date by running
    git submodule update --init --recursive
--------------------------------------------------------------------------
"
        )
    });

    eprintln!("Testing dbc files in directory: {dir_display}");
    for dbc_path in dir_content {
        let path = dbc_path.unwrap().path();
        if path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("dbc"))
        {
            parse_one_file(path);
        }
    }
}

/// Parse a single DBC file and assert a snapshot of the result.
fn parse_one_file(path: impl AsRef<Path>) {
    let path = path.as_ref();
    eprintln!("Testing DBC file: {}", path.display());
    let file_name = path.file_stem().unwrap().to_string_lossy().to_string();
    let buffer = fs::read(path).unwrap();
    if let Some(buffer) = decode_cp1252(&buffer) {
        match DbcParser::parse(Rule::file, &buffer) {
            Ok(pairs) => assert_debug_snapshot!(file_name, pairs),
            Err(e) => panic!("Failed to parse {file_name}.dbc: {e:#?}"),
        }
    } else {
        panic!("Failed to decode {file_name}.dbc as cp1252");
    }
}
