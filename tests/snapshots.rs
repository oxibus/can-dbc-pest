#![cfg(feature = "encoding")]

use std::borrow::Cow;
use std::fs;
use std::path::Path;

use can_dbc_pest::{decode_cp1252, DbcParser, Rule};
use insta::{assert_debug_snapshot, with_settings};
use pest::Parser;

/// Test parsing all DBC files
#[test]
fn test_cantools_dbc_files() {
    let test_dirs = [
        (
            "tests/shared-test-files/dbc-cantools",
            "dbc-cantools",
            true,
            true,
        ),
        ("tests/shared-test-files/canpy", "canpy", true, true),
        ("tests/opendbc/opendbc/dbc", "opendbc", false, false),
    ];
    for (path, file_name, snapshot, cp1251) in test_dirs {
        let path = Path::new(path);
        let snapshot_path = String::from("snapshots-") + file_name;
        // snapshots should go to /tests/snapshots-* directory
        with_settings! {
            { omit_expression => true,
              snapshot_path => snapshot_path ,
              prepend_module_to_snapshot => false },
            {
                test_dbc_files(path, snapshot, cp1251);
            }
        }
    }
}

/// Test parsing all DBC files in the given directory.
fn test_dbc_files(dir: impl AsRef<Path>, snapshot: bool, cp1251: bool) {
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
            parse_one_file(path, snapshot, cp1251);
        }
    }
}

/// Parse a single DBC file and assert a snapshot of the result.
fn parse_one_file(path: impl AsRef<Path>, snapshot: bool, cp1251: bool) {
    let path = path.as_ref();
    eprintln!("Testing DBC file: {}", path.display());
    let file_name = path.file_stem().unwrap().to_string_lossy().to_string();
    let buffer = fs::read(path).unwrap();
    let buffer = if cp1251 {
        decode_cp1252(&buffer)
            .unwrap_or_else(|| panic!("Failed to decode {} as cp1252", path.display()))
    } else {
        Cow::Borrowed(
            std::str::from_utf8(&buffer)
                .unwrap_or_else(|_| panic!("Failed to decode {} as utf-8", path.display())),
        )
    };

    match DbcParser::parse(Rule::file, &buffer) {
        Ok(pairs) => {
            if std::env::var("SKIP_INSTA").is_err()
                && (snapshot || std::env::var("FORCE_INSTA").is_ok())
            {
                assert_debug_snapshot!(file_name, pairs);
            }
        }
        Err(e) => panic!("Failed to parse {file_name}.dbc: {e:#?}"),
    }
}
