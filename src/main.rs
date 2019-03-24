extern crate xmlrpc;

use xmlrpc::Request;
use std::collections::btree_map::BTreeMap;

const SERVER_URL: &str = "https://koji.fedoraproject.org/kojihub/";


#[derive(Debug)]
struct Build {
    build_id: i32,
    completion_time: String,
    completion_ts: f64,
    creation_event_id: i32,
    creation_time: String,
    creation_ts: f64,
    epoch: Option<String>,
    // extra: BuildExtra,
    id: i32,
    name: String,
    nvr: String,
    owner_id: i32,
    owner_name: String,
    package_id: i32,
    package_name: String,
    release: String,
    source: String,
    start_time: String,
    start_ts: f64,
    state: i32,
    task_id: i32,
    version: String,
    volume_id: i32,
    volume_name: String,
}


fn i32_from_struct(data: &BTreeMap<String, xmlrpc::Value>, key: &str) -> Result<i32, String> {
    match data.get(key) {
        Some(value) => match value.as_i32() {
            Some(value) => Ok(value),
            None => Err(format!("Expected Integer, got zilch for '{}'", key)),
        },
        None => Err(format!("Expected Integer, got zilch for '{}'", key)),
    }
}


fn f64_from_struct(data: &BTreeMap<String, xmlrpc::Value>, key: &str) -> Result<f64, String> {
    match data.get(key) {
        Some(value) => match value.as_f64() {
            Some(value) => Ok(value),
            None => Err(format!("Expected Double, got zilch for '{}'", key)),
        },
        None => Err(format!("Expected Double, got zilch for '{}'", key)),
    }
}


fn string_from_struct(data: &BTreeMap<String, xmlrpc::Value>, key: &str) -> Result<String, String> {
    match data.get(key) {
        Some(value) => match value.as_str() {
            Some(value) => Ok(String::from(value)),
            None => Err(format!("Expected String, got zilch for '{}'", key)),
        },
        None => Err(format!("Expected String, got zilch for '{}'", key)),
    }
}


impl Build {
    fn from_response(response: Result<xmlrpc::Value, xmlrpc::Error>) -> Result<Build, String> {
        match response {
            Ok(value) => {
                let data: &BTreeMap<String, xmlrpc::Value> = match value.as_struct() {
                    Some(data) => data,
                    None => { return Err(String::from("Empty response.")); }
                };

                Ok(Build {
                    build_id: i32_from_struct(data, "build_id")?,
                    completion_time: string_from_struct(data, "completion_time")?,
                    completion_ts: f64_from_struct(data, "completion_ts")?,
                    creation_event_id: i32_from_struct(data, "creation_event_id")?,
                    creation_time: string_from_struct(data, "creation_time")?,
                    creation_ts: f64_from_struct(data, "creation_ts")?,
                    epoch: string_from_struct(data, "epoch").ok(),
                    id: i32_from_struct(data, "id")?,
                    name: string_from_struct(data, "name")?,
                    nvr: string_from_struct(data, "nvr")?,
                    owner_id: i32_from_struct(data, "owner_id")?,
                    owner_name: string_from_struct(data, "owner_name")?,
                    package_id: i32_from_struct(data, "package_id")?,
                    package_name: string_from_struct(data, "package_name")?,
                    release: string_from_struct(data, "release")?,
                    source: string_from_struct(data, "source")?,
                    start_time: string_from_struct(data, "start_time")?,
                    start_ts: f64_from_struct(data, "start_ts")?,
                    state: i32_from_struct(data, "state")?,
                    task_id: i32_from_struct(data, "task_id")?,
                    version: string_from_struct(data, "version")?,
                    volume_id: i32_from_struct(data, "volume_id")?,
                    volume_name: string_from_struct(data, "volume_name")?,
                })
            }
            Err(error) => { return Err(format!("{:?}", error)); }
        }
    }
}


struct KojiService {
    url: String,
}


impl KojiService {
    fn new(url: String) -> KojiService {
        KojiService { url }
    }

    fn get_build(&self, nvr: &str) -> Result<Build, String> {
        let build_req = Request::new("getBuild").arg(nvr);
        let build_res = build_req.call_url(&self.url);
        Build::from_response(build_res)
    }
}


fn main() {
    let koji = KojiService::new(String::from(SERVER_URL));
    let nvr = String::from("syncthing-1.1.0-1.fc30");

    let build = koji.get_build(&nvr);

    println!("{:#?}", build);
}
