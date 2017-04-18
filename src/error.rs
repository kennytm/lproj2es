//! Error types for `lproj2es`.

use std::path::PathBuf;
use serde_json::Value;

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Plist(::plist::Error);
        Hyper(::hyper::error::Error);
        Json(::serde_json::Error);
    }

    errors {
        CreateIndex {
            description("cannot create index")
        }
        CreateIndexUnexpectedReply(reply: Value) {
            description("cannot create index")
            display("cannot create index, unexpected reply from Elasticsearch: {}", reply)
        }
        ReadLproj(lproj: PathBuf) {
            description("cannot read *.lproj")
            display("cannot read {}", lproj.display())
        }
        InvalidPlist {
            description("*.strings file is not a valid plist")
        }
        IndexTranslations(bundle: PathBuf) {
            description("cannot index translations")
            display("cannot index translations for bundle {}", bundle.display())
        }
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