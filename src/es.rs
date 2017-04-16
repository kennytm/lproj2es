//! Elasticsearch driver.

use std::io::Write;
use rs_es::Client;
use rs_es::operations::bulk::Action;
use serde_json::{to_vec, from_reader, Value};

use error::{ErrorKind, Result, ResultExt};

/// Elasticsearch client.
pub struct Es<'a> {
    client: Client,
    index: &'a str,
    type_: &'a str,
}

impl<'a> Es<'a> {
    /// Constructs a new Elasticsearch client.
    pub fn new(base: &str, index: &'a str, type_: &'a str) -> Result<Es<'a>> {
        Ok(Es {
            client: Client::new(base).chain_err(|| ErrorKind::NewEsClient)?,
            index: index,
            type_: type_,
        })
    }

    /// Creates the "localization" index.
    pub fn create_index(&mut self, shards: u32, replicas: u32) -> Result<()> {
        // FIXME: Switch to rs-es API once create_index is supported.
        use hyper::client::{Client, Body};
        use hyper::header::ContentType;

        let url = self.client.full_url(self.index);
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

        let client = Client::new();
        let resp = client
            .put(&url)
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
    pub fn add_translations<I>(&mut self, translations: I) -> Result<usize> where I: Iterator<Item=Value> {
        let actions = translations.map(Action::index).collect::<Vec<_>>();
        if actions.is_empty() {
            // we need to special-case this, otherwise we will cause Elasticsearch to go NPE (HTTP 500).
            return Ok(0);
        }

        let result = self.client
            .bulk(&actions)
            .with_index(self.index)
            .with_doc_type(self.type_)
            .send()?;
        Ok(result.items.into_iter().filter(|r| r.inner.status == 201).count())
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