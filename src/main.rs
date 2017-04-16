#[macro_use] extern crate lazy_static;
#[macro_use] extern crate maplit;
#[macro_use] extern crate structopt_derive;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate serde_json;
extern crate walkdir;
extern crate rs_es;
extern crate hyper;
extern crate structopt;
extern crate plist;
extern crate progress;
extern crate clap;

macro_rules! eprintln {
    ($($e:expr),*) => {
        writeln!(::std::io::stderr(), $($e),*).unwrap();
    }
}

mod lproj;
mod es;
mod error;
mod options;
mod locales;

use std::io::Write;
use std::default::Default;
use std::time::{Instant, Duration};
use std::fmt::{self, Display, Formatter};
use progress::Bar;

use options::Options;
use lproj::{scan_localized_bundles, LocalizedBundle};
use locales::locale_id;
use es::Es;
use error::{ErrorKind, ResultExt, Result};

quick_main!(run);

fn run() -> Result<()> {
    let opt = Options::parse();

    let start_time = Instant::now();

    eprintln!("Connecting to ElasticSearch cluster at `{}`.", opt.base);
    let mut es = Es::new(&opt.base, &opt.index, &opt.type_)?;
    es.create_index(opt.shards, opt.replicas)?;

    eprintln!("Scanning for localized bundles from `{}`...", opt.root);
    let localized_bundles = scan_localized_bundles(opt.root);

    let mut progress_bar = ProgressBar::new(localized_bundles.len());
    eprintln!("Found {} localized bundles.", progress_bar.total);

    progress_bar.start();
    // written as a `fold` to allow for future parallelization.
    let total_count = localized_bundles.into_iter().fold(Ok(0), |prev_count, (bundle_path, localizations)| -> Result<usize> {
        let prev_count = prev_count?;

        let mut bundle = LocalizedBundle::default();
        for localization in &localizations {
            let lproj_path = bundle_path.join(localization);
            let loc_id = locale_id(localization);
            bundle.read_lproj(loc_id, &lproj_path).chain_err(|| ErrorKind::ReadLproj(lproj_path))?;
        }

        let translations = bundle.into_iter(&bundle_path);
        let count = es.add_translations(translations).chain_err(|| ErrorKind::IndexTranslations(bundle_path))?;
        progress_bar.add_one();

        Ok(prev_count + count)
    })?;

    let duration = start_time.elapsed();
    eprintln!("Finished, imported {} translations in {}", total_count, PrettyDuration(duration));

    Ok(())
}

/// Wrapper to print a duration in the format `59s` or `2m30s`.
struct PrettyDuration(Duration);

impl Display for PrettyDuration {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let secs = self.0.as_secs();
        if secs > 60 {
            write!(f, "{}m{:02}s", secs/60, secs%60)
        } else {
            write!(f, "{}s", secs)
        }
    }
}

/// Wrapper to print progress.
struct ProgressBar {
    bar: Bar,
    total: usize,
    count: usize,
}

impl ProgressBar {
    fn new(total: usize) -> ProgressBar {
        ProgressBar {
            bar: Bar::new(),
            total: total,
            count: 0,
        }
    }

    fn add_one(&mut self) {
        self.count += 1;
        self.bar.reach_percent((self.count * 100 / self.total) as i32);
        if self.count >= self.total {
            self.bar.jobs_done();
        }
    }

    fn start(&mut self) {
        self.bar.set_job_title("Indexing...");
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