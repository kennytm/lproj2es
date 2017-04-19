#[macro_use] extern crate lazy_static;
#[macro_use] extern crate maplit;
#[macro_use] extern crate structopt_derive;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate mime;
extern crate walkdir;
extern crate hyper;
extern crate structopt;
extern crate plist;
extern crate clap;
extern crate rayon;
extern crate pbr;

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
use std::sync::Mutex;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use pbr::ProgressBar;

use options::Options;
use lproj::{scan_localized_bundles, LocalizedBundle};
use locales::locale_id;
use es::Es;
use error::{ErrorKind, ResultExt, Result};

quick_main!(run);

fn run() -> Result<()> {
    let opt = Options::parse();

    let start_time = Instant::now();

    eprintln!("Connecting to Elasticsearch cluster at `{}`.", opt.base);
    let es = Es::new(opt.base, &opt.index, &opt.type_);
    es.create_index(opt.shards, opt.replicas)?;

    eprintln!("Scanning for localized bundles from `{}`...", opt.root);
    let localized_bundles = scan_localized_bundles(opt.root);

    let mut progress_bar = ProgressBar::new(localized_bundles.len() as u64);
    progress_bar.set_width(Some(100));
    progress_bar.set_max_refresh_rate(Some(Duration::from_millis(500)));
    let mut progress_bar = Mutex::new(progress_bar);

    let total_count: Result<usize> = localized_bundles.into_par_iter().map(|(bundle_path, localizations)| -> Result<usize> {
        let mut bundle = LocalizedBundle::default();
        for localization in &localizations {
            let lproj_path = bundle_path.join(localization);
            let loc_id = locale_id(localization);
            bundle.read_lproj(loc_id, &lproj_path).chain_err(|| ErrorKind::ReadLproj(lproj_path))?;
        }

        let translations = bundle.into_iter(&bundle_path);
        let count = es.add_translations(translations).chain_err(|| ErrorKind::IndexTranslations(bundle_path))?;
        {
            let mut lock = progress_bar.lock().unwrap();
            lock.inc();
        }

        Ok(count)
    }).sum();
    let total_count = total_count?;

    let duration = start_time.elapsed();
    let finish_msg = format!("Finished, imported {} translations in {}", total_count, PrettyDuration(duration));
    progress_bar.get_mut().unwrap().finish_println(&finish_msg);

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