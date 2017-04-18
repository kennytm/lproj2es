//! Parse command line options.

use clap::AppSettings;
use hyper::Url;
use structopt::StructOpt;

/// Command line options.
#[derive(StructOpt, Debug)]
pub struct Options {
    #[structopt(help="Root directory to scan for localized bundles")]
    pub root: String,

    #[structopt(short="-u", long="--url", help="Acesss point of the Elasticsearch cluster", default_value="http://127.0.0.1:9200")]
    pub base: Url,

    #[structopt(short="-i", long="--index", help="Name of the index", default_value="localizations")]
    pub index: String,

    #[structopt(short="-t", long="--type", help="Name of the type", default_value="ios")]
    pub type_: String,

    #[structopt(long="--shards", help="Number of shards of the new index", default_value="1")]
    pub shards: u32,

    #[structopt(long="--replicas", help="Number of replicas of the new index", default_value="1")]
    pub replicas: u32,
}

impl Options {
    pub fn parse() -> Self {
        let app = Self::clap().global_settings(&[
            AppSettings::ArgRequiredElseHelp,
            AppSettings::DeriveDisplayOrder,
        ]);
        Self::from_clap(app.get_matches())
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