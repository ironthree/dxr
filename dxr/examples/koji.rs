// TODO: move code from this file to koji-rs

#![warn(missing_debug_implementations)]
#![warn(clippy::unwrap_used)]

use dxr::{Call, Client, FromDXR, ToDXR};
use url::Url;

#[derive(Debug, FromDXR, ToDXR)]
pub struct Build {
    pub build_id: i32,
    //cg_id: Option<?>,
    pub completion_time: String,
    pub completion_ts: f64,
    pub creation_event_id: i32,
    pub creation_time: String,
    pub creation_ts: f64,
    pub epoch: Option<i32>,
    //extra: HashMap<String, Value>,
    pub id: i32,
    pub name: String,
    pub nvr: String,
    pub owner_id: i32,
    pub owner_name: String,
    pub package_id: i32,
    pub package_name: String,
    pub release: String,
    pub source: String,
    pub start_time: String,
    pub start_ts: f64,
    pub state: i32,
    pub task_id: i32,
    pub version: String,
    pub volume_id: i32,
    pub volume_name: String,
    //cg_name: Option<?>,
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let url = Url::parse("https://koji.fedoraproject.org/kojihub/").expect("Failed to parse hardcoded URL.");
    let client = Client::new(url);

    let request: Call<_, Build> = Call::new(String::from("getBuild"), vec![String::from("syncthing-1.1.0-1.fc30")]);
    let result = client.call(request).await.map_err(|error| error.to_string())?;

    // print query result
    println!("{:#?}", result);

    Ok(())
}
