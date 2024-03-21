use crate::config::GrafanaConfig;
use regex::Regex;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Grafana {
    instances: Vec<GrafanaInstance>,
}

#[derive(Debug, Clone)]
pub struct GrafanaInstance {
    name: String,
    url: String,
}

impl Grafana {
    pub fn new(config: Vec<GrafanaConfig>) -> Self {
        let mut grafana: Vec<GrafanaInstance> = Vec::new();

        for instance in config {
            let grafana_instance = GrafanaInstance {
                name: instance.name,
                url: instance.url,
            };

            grafana.push(grafana_instance)
        }

        Self { instances: grafana }
    }

    pub fn get_grafana_update(&self) {
        for instance in self.instances.clone() {
            let version = get_grafana_version(instance);

            get_grafana_latest_version(version);
        }
    }
}

#[::tokio::main]
async fn get_grafana_version(instance: GrafanaInstance) -> String {
    const GRAFANA_API_PATH: &str = "/api/health";

    let response = reqwest::get(format!("{}{}", instance.url, GRAFANA_API_PATH))
        .await
        .unwrap();

    let body = response.text().await.unwrap();
    let json_data = serde_json::from_str::<Value>(&body).unwrap();

    json_data.get("version").unwrap().to_string()
}

#[::tokio::main]
async fn get_grafana_latest_version(curr_version: String) {
    const GRAFANA_CHANGELOG_URL: &str =
        "https://raw.githubusercontent.com/grafana/grafana/main/CHANGELOG.md";

    let response = reqwest::get(format!("{}", GRAFANA_CHANGELOG_URL))
        .await
        .unwrap();

    let body = response.text().await.unwrap();

    let re = Regex::new(r"^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)(-[0-9A-Za-z\.-]+)?(\+[0-9A-Za-z\.-]+)?$|^Unreleased$").unwrap();
    let caps = re.captures(&body).unwrap();

    println!("{:?}", caps);
}
