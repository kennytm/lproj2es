#[macro_use] extern crate structopt_derive;
#[macro_use] extern crate iron;
#[macro_use] extern crate serde_json;
extern crate mount;
extern crate structopt;
extern crate hyper;
extern crate serde;
extern crate params;
extern crate staticfile;
extern crate url;

use std::net::SocketAddr;
use std::sync::Arc;
use std::mem::replace;
use std::path::Path;
use iron::prelude::*;
use iron::headers::ContentType;
use iron::status;
use mount::Mount;
use structopt::StructOpt;
use hyper::{Client, Url};
use hyper::client::{Body, RequestBuilder};
use hyper::header::{Authorization, Basic};
use hyper::method::Method;
use serde::Serialize;
use serde_json::{Value, from_reader, to_vec};
use params::Params;
use params::Value::String as PVString;
use staticfile::Static;
use url::percent_encoding::percent_decode;

#[derive(StructOpt)]
struct Options {
    #[structopt(short="-u", long="--url", help="Acesss point of the Elasticsearch cluster", default_value="http://127.0.0.1:9200")]
    pub base: Url,

    #[structopt(short="-i", long="--index", help="Name of the index", default_value="localizations")]
    pub index: String,

    #[structopt(short="-t", long="--type", help="Name of the type", default_value="ios")]
    pub type_: String,

    #[structopt(short="-l", long="--listen", help="Listerning address for the HTTP server", default_value="127.0.0.1:59447")]
    pub listen: SocketAddr,
}

struct Searcher {
    client: Client,
    base: Url,
    authorization: Option<Authorization<Basic>>,
    index: String,
    type_: String,
}

impl Searcher {
    fn request(&self, method: Method, path: &str) -> RequestBuilder {
        let url = self.base.join(path).unwrap();
        let mut req = self.client.request(method, url);
        if let Some(ref authorization) = self.authorization {
            req = req.header(authorization.clone());
        }
        req
    }

    fn list_languages(&self) -> IronResult<Vec<String>> {
        let path = format!("/{}/_mappings/{}", self.index, self.type_);
        let resp = itry!(self.request(Method::Get, &path).send());
        let content: Value = itry!(from_reader(resp));
        let properties = content[&self.index]["mappings"][&self.type_]["properties"].as_object();
        Ok(properties.map(|p| p.keys().filter(|s| s.contains('_')).cloned().collect()).unwrap_or_else(Vec::new))
    }

    fn search<'a, I>(&self, source: &str, targets: I, keyword: &str) -> IronResult<Vec<Value>>
        where I: Iterator<Item=&'a str>
    {
        let path = format!("/{}/{}/_search", self.index, self.type_);
        let query = construct_search_query(source, targets, keyword);
        let body = to_vec(&query).unwrap();
        let resp = itry!(self.request(Method::Post, &path)
            .header(ContentType::json())
            .body(Body::BufBody(&body, body.len()))
            .send());
        let content = itry!(from_reader(resp));
        Ok(parse_search_result(content))
    }
}

fn construct_search_query<'a, I>(source: &str, targets: I, keyword: &str) -> Value
        where I: Iterator<Item=&'a str>
{
    let mut aggs = json!({"entry": {"top_hits": {"size": 1}}});
    for target in targets {
        aggs = json!({
            target: {
                "terms": {
                    "field": format!("{}.keyword", target),
                    "missing": "",
                },
                "aggregations": aggs,
            },
        });
    }

    json!({
        "query": {"match": {source: keyword}},
        "size": 0,
        "aggregations": {
            source: {
                "terms": {
                    "field": format!("{}.keyword", source),
                    "collect_mode": "breadth_first",
                    "size": 100,
                },
                "aggregations": aggs,
            },
        },
    })
}

fn parse_search_result(mut content: Value) -> Vec<Value> {
    let mut result = Vec::new();

    macro_rules! try_opt {
        ($e:expr) => {
            match $e {
                Some(e) => e,
                None => return result,
            }
        };
    }

    let mut stack = vec![&mut content["aggregations"]];
    while let Some(aggs) = stack.pop().and_then(Value::as_object_mut) {
        let doc_count = aggs.get("doc_count").and_then(Value::as_u64).unwrap_or(0);
        for (key, value) in aggs.iter_mut() {
            match &**key {
                "key" | "doc_count" => {},
                "entry" => {
                    let mut entry = replace(&mut value["hits"]["hits"][0], Value::Null);
                    entry["_count"] = Value::from(doc_count);
                    result.push(entry);
                },
                _ => stack.extend(try_opt!(value["buckets"].as_array_mut()).iter_mut()),
            }
        }
    }

    result
}

fn reply_json<T: ?Sized + Serialize>(r: &T) -> IronResult<Response> {
    let body = itry!(to_vec(r));
    let mut resp = Response::with((status::Ok, body));
    resp.headers.set(ContentType::json());
    Ok(resp)
}

fn decode(s: &str) -> String {
    percent_decode(s.as_bytes()).decode_utf8_lossy().into_owned()
}

fn main() {
    let opts = Options::from_args();

    let mut base = opts.base;
    let authorization = match (base.username(), base.password()) {
        ("", None) => None,
        (username, password) => Some(Authorization(Basic {
            username: decode(username),
            password: password.map(decode),
        })),
    };
    let _ = base.set_username("");
    let _ = base.set_password(None);

    let search_searcher = Arc::new(Searcher {
        client: Client::new(),
        base: base,
        authorization: authorization,
        index: opts.index,
        type_: opts.type_,
    });
    let languages_searcher = Arc::clone(&search_searcher);

    let mut mount = Mount::new();
    mount.mount("/languages", move |_: &mut Request| reply_json(&languages_searcher.list_languages()?));
    mount.mount("/search", move |req: &mut Request| -> IronResult<Response> {
        let params = req.get::<Params>().unwrap();
        let (source, target, keyword) = match (params.get("f"), params.get("t"), params.get("k")) {
            (Some(&PVString(ref f)), Some(&PVString(ref t)), Some(&PVString(ref k))) => (f, t, k),
            _ => return Ok(Response::with((status::BadRequest, "[]"))),
        };
        reply_json(&search_searcher.search(source, target.split(','), keyword)?)
    });
    if cfg!(debug_assertions) {
        mount.mount("/", Static::new(Path::new(file!()).with_file_name("home.html")));
    } else {
        mount.mount("/", move |_: &mut Request| {
            let mut resp = Response::with((status::Ok, include_str!("home.html")));
            resp.headers.set(ContentType::html());
            Ok(resp)
        });
    }


    println!("Listening on {}", opts.listen);
    Iron::new(mount).http(opts.listen).unwrap();
}


