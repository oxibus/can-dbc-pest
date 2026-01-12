#![cfg(feature = "encodings")]

use std::borrow::Cow;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::{env, fs};

use can_dbc_pest::{decode_cp1252, DbcParser, Parser as _, Rule};
use insta::{assert_debug_snapshot, with_settings};
use test_each_file::test_each_path;

struct TestConfig {
    test_root: &'static str,
    snapshot_suffix: &'static str,
    use_cp1251: bool,
    create_snapshot: bool,
}

static TEST_DIRS: &[TestConfig] = &[
    TestConfig {
        test_root: "shared-test-files",
        snapshot_suffix: "",
        use_cp1251: true,
        create_snapshot: true,
    },
    TestConfig {
        test_root: "opendbc/opendbc/dbc",
        snapshot_suffix: "opendbc",
        use_cp1251: false,
        create_snapshot: false,
    },
];

test_each_path! { for ["dbc"] in "./tests/fixtures/opendbc/opendbc/dbc" as dbc => parse_one_file }
test_each_path! { for ["dbc"] in "./tests/fixtures/shared-test-files" as shared => parse_one_file }
// upper case extension
test_each_path! { for ["DBC"] in "./tests/fixtures/shared-test-files" as shared2 => parse_one_file }

type DecoderFn = Box<dyn Fn(&[u8]) -> Cow<'_, str>>;

/// Get snapshot path (if snapshot should be created) and a decoding
/// function for a test file path
fn get_test_info(path: &Path) -> Option<(Option<PathBuf>, DecoderFn)> {
    if !path
        .extension()
        .unwrap_or_default()
        .eq_ignore_ascii_case("dbc")
    {
        return None;
    }
    let path_str = path.display().to_string();
    let parent = path.parent().unwrap();
    for item in TEST_DIRS {
        // Ensure slashes are there for easier matching
        let test_root = format!("/{}/", item.test_root);
        let mut path_dir = parent.to_str().unwrap().to_string();
        if !path_dir.ends_with('/') {
            path_dir.push('/');
        }
        if let Some(pos) = path_dir.find(&test_root) {
            let snapshot = (if item.create_snapshot {
                Some("snapshots")
            } else if env::var("FORCE_INSTA").is_ok() {
                Some("snapshots-forced") // this dir is .gitignored
            } else {
                None
            })
            .map(|v| {
                PathBuf::from(v)
                    .join(item.snapshot_suffix)
                    .join(&path_dir[pos + test_root.len()..])
            });

            let decoder: DecoderFn = if item.use_cp1251 {
                Box::new(move |v: &[u8]| {
                    decode_cp1252(v).unwrap_or_else(|| panic!("Cannot decode {path_str} as cp1252"))
                })
            } else {
                Box::new(move |v: &[u8]| {
                    std::str::from_utf8(v)
                        .unwrap_or_else(|_| panic!("Cannot decode {path_str} as utf-8"))
                        .into()
                })
            };

            return Some((snapshot, decoder));
        }
    }
    panic!("Unknown test directory: {path_str}");
}

/// Test parsing all DBC files
#[test]
fn test_if_submodules_are_present() {
    for test in TEST_DIRS {
        let dir = Path::new("./tests/fixtures").join(test.test_root);
        fs::read_dir(&dir)
            .and_then(|v| {
                v.into_iter()
                    .next()
                    .map(|_| ())
                    .ok_or_else(|| Error::new(ErrorKind::NotFound, "No files or dirs found"))
            })
            .unwrap_or_else(|e| {
                let dir_display = dir.display();
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
    }
}

/// Parse a single DBC file and assert a snapshot of the result.
fn parse_one_file([path]: [&Path; 1]) {
    let Some((snapshot_path, decoder)) = get_test_info(path) else {
        return;
    };

    let file_name = path.file_stem().unwrap().to_string_lossy().to_string();
    let buffer = fs::read(path).unwrap();
    let buffer = decoder(&buffer);

    match DbcParser::parse(Rule::file, &buffer) {
        Ok(result) => {
            if let Some(snapshot_path) = snapshot_path {
                with_settings! {
                    {
                        omit_expression => true,
                        snapshot_path => snapshot_path,
                        prepend_module_to_snapshot => false
                    },
                    {
                        assert_debug_snapshot!(file_name, result);
                    }
                }
            }
        }
        Err(e) => panic!("Failed to parse {file_name}.dbc: {e:#?}"),
    }
}
