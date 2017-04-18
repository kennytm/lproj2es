//! Elasticsearch driver.

use std::io::Write;
use hyper::Url;
use hyper::client::{Client, Body};
use hyper::header::ContentType;
use serde_json::{to_vec, to_writer, from_reader, Value};

use error::{ErrorKind, Result, ResultExt};

/// Elasticsearch client.
pub struct Es<'a> {
    client: Client,
    base: Url,
    index: &'a str,
    type_: &'a str,
}

impl<'a> Es<'a> {
    /// Constructs a new Elasticsearch client.
    pub fn new(base: Url, index: &'a str, type_: &'a str) -> Es<'a> {
        Es {
            client: Client::new(),
            base: base,
            index: index,
            type_: type_,
        }
    }

    /// Creates the "localization" index.
    pub fn create_index(&self, shards: u32, replicas: u32) -> Result<()> {
        let body = to_vec(&json!({
            "settings": {
                "number_of_shards": shards,
                "number_of_replicas": replicas,
            },
            "mappings": {
                self.type_: {
                    "_all": {"enabled": false},
                    "dynamic": true,
                },
            },
        })).unwrap();

        let resp = self.client
            .put(self.base.join(self.index).unwrap())
            .header(ContentType::json())
            .body(Body::BufBody(&body, body.len()))
            .send().chain_err(|| ErrorKind::CreateIndex)?;

        let content: Value = from_reader(resp).chain_err(|| ErrorKind::CreateIndex)?;

        if content["acknowledged"].as_bool() == Some(true) {
            Ok(())
        } else if content["error"]["type"].as_str() == Some("index_already_exists_exception") {
            let error_reason = content["error"]["reason"].as_str().unwrap_or("localization index already exists");
            eprintln!("warning: {}", error_reason);
            Ok(())
        } else {
            bail!(ErrorKind::CreateIndexUnexpectedReply(content))
        }
    }

    /// Inserts an iterator of JSON values into the localization index.
    ///
    /// Returns the number of entries successfully added.
    pub fn add_translations<I>(&self, translations: I) -> Result<usize> where I: Iterator<Item=Value> {
        let index = to_vec(&json!({
            "index": {
                "_index": self.index,
                "_type": self.type_,
            },
        })).unwrap();

        let mut bulk = Vec::new();
        for translation in translations {
            bulk.extend(&index);
            bulk.push(b'\n');
            to_writer(&mut bulk, &translation)?;
            bulk.push(b'\n');
        }
        if bulk.is_empty() {
            // we need to special-case this, otherwise we will cause Elasticsearch to go NPE (HTTP 500).
            return Ok(0);
        }

        let result = self.client
            .post(self.base.join("_bulk").unwrap())
            .header(ContentType(mime!(Application/("x-ndjson"))))
            .body(Body::BufBody(&bulk, bulk.len()))
            .send()?;

        let result: Value = from_reader(result)?;

        let items = result["items"].as_array();
        Ok(items.map(|a| a.iter().filter(|r| r["index"]["status"].as_i64() == Some(201)).count()).unwrap_or(0))
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