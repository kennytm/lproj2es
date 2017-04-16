//! Find all descendant paths containing *.lproj from the root directory.
//!
//! If the root directory structure is like:
//!
//! ```text
//! /root/
//!     Foo.app/
//!         Base.lproj/...
//!         English.lproj/...
//!     Bar/
//!         a.bundle/
//!             en_US.lproj/...
//!             fr_FR.lproj/...
//! ```
//!
//! then running `find_lproj_containers("/root")` will return a hash map of
//! `{"/root/Foo.app/": ["Base.lproj", "English.lproj"], "/root/Bar/a.bundle": ["en_US.lproj", "fr_FR.lproj"]}`.

use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use std::collections::{BTreeMap, HashMap, hash_map};
use std::fs::{File, read_dir};
use walkdir::{WalkDir, WalkDirIterator};
use serde_json::{Value, to_string_pretty};
use plist::Plist;

use error::{Result, ErrorKind};

/// Finds all localized bundles inside the given directory.
pub fn scan_localized_bundles<P: AsRef<Path>>(root: P) -> BTreeMap<PathBuf, Vec<String>> {
    let lproj = Some(OsStr::new("lproj"));
    let root = root.as_ref();
    let mut result = BTreeMap::new();

    let mut walker = WalkDir::new(root).into_iter().filter_entry(|e| e.file_type().is_dir());
    while let Some(entry) = walker.next() {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => {
                walker.skip_current_dir(); // probably "no permission" or something like that.
                continue;
            },
        };
        let path = entry.path();
        if path.extension() != lproj {
            continue;
        }

        let container = path.parent().unwrap().to_owned();
        result.entry(container).or_insert_with(Vec::new).push(entry.file_name().to_string_lossy().into_owned());
        walker.skip_current_dir();
    }

    result
}

#[test]
fn test_scan_localized_bundles() {

}


/// Converts a plist into a JSON value.
///
/// Panics if the input cannot be converted to JSON (but it should not happen with strings/stringsdict files).
fn into_json_value(plist: Plist) -> Value {
    use serde_json::Number;

    match plist {
        Plist::Array(a) => Value::Array(a.into_iter().map(into_json_value).collect()),
        Plist::Dictionary(d) => Value::Object(d.into_iter().map(|(k, v)| (k, into_json_value(v))).collect()),
        Plist::Boolean(b) => Value::Bool(b),
        Plist::Real(f) if f.is_finite() => Value::Number(Number::from_f64(f).unwrap()),
        Plist::Integer(i) => Value::Number(Number::from(i)),
        Plist::String(s) => Value::String(s),
        v => panic!("unexpected plist type {:?} with no JSON correspondance, cannot convert", v),
    }
}

/// Converts the JSON value into a string.
fn into_es_string(value: Value) -> String {
    match value {
        Value::String(s) => s,
        v => to_string_pretty(&v).unwrap(),
    }
}

/// Tuple of the file name of the `*.strings` file and the localization key.
#[derive(PartialEq, Eq, Hash)]
struct Key {
    file: String,
    key: String,
}

/// Stores all localized strings in a bundle.
#[derive(Default)]
pub struct LocalizedBundle<'a>(HashMap<Key, HashMap<&'a str, String>>);
// {key => {locale => value}}

impl<'a> LocalizedBundle<'a> {
    /// Adds a key-value pair in the given locale.
    fn add_entry(&mut self, locale_id: &'a str, key: Key, value: Value) {
        let values = self.0.entry(key).or_insert_with(|| HashMap::new());
        values.insert(locale_id, into_es_string(value));
    }

    /// Parses a single plist file of a given locale. All key-value pairs in the file will be stored.
    fn parse_plist_file(&mut self, locale_id: &'a str, path: &Path) -> Result<()> {
        let file_name = path.file_name().unwrap().to_string_lossy().into_owned();

        let file = File::open(path)?;
        let plist = match Plist::read(file) {
            Ok(p) => p,
            Err(_) => return Ok(()), // ignore all errors for now. https://github.com/ebarnard/rust-plist/issues/20
        };

        if let Plist::Dictionary(dict) = plist {
            for (key, value) in dict {
                let key = Key { file: file_name.clone(), key: key };
                self.add_entry(locale_id, key, into_json_value(value));
            }
        } else {
            bail!(ErrorKind::InvalidPlist);
        }

        Ok(())
    }

    /// Parses a `*.strings` file of a given locale. If there is an accompanying `*.stringsdict` file, it will be parsed
    /// too.
    fn parse_strings_file(&mut self, locale_id: &'a str, strings_file: &Path) -> Result<()> {
        self.parse_plist_file(locale_id, strings_file)?;
        let strings_dict_file = strings_file.with_extension("stringsdict");
        if strings_dict_file.is_file() {
            self.parse_plist_file(locale_id, &strings_dict_file)?;
        }
        Ok(())
    }

    /// Scans for all `*.strings` files inside an `*.lproj` directory.
    pub fn read_lproj(&mut self, locale_id: &'a str, lproj_path: &Path) -> Result<()> {
        let strings_extension = Some(OsStr::new("strings"));
        for entry in read_dir(lproj_path)? {
            let path = entry?.path();
            if path.extension() == strings_extension {
                self.parse_strings_file(locale_id, &path)?;
            }
        }
        Ok(())
    }

    /// After all `*.lproj`s are read, converts this instance into an iterator to read the entries.
    pub fn into_iter(self, filename: &Path) -> LocalizedBundleIntoIter<'a> {
        LocalizedBundleIntoIter {
            filename: filename.to_string_lossy().into_owned(),
            it: self.0.into_iter(),
        }
    }
}


pub struct LocalizedBundleIntoIter<'a> {
    filename: String,
    it: hash_map::IntoIter<Key, HashMap<&'a str, String>>,
}

impl<'a> Iterator for LocalizedBundleIntoIter<'a> {
    type Item = Value;

    fn next(&mut self) -> Option<Value> {
        self.it.next().map(|(k, mut v)| {
            v.insert("BUNDLE", self.filename.clone());
            v.insert("FILE", k.file);
            v.insert("KEY", k.key);
            json!(v)
        })
    }
}

/*

Copyright 2017 kennytm

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the Software without restriction, including without limitation the
rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit
persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the
Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE
WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

*/