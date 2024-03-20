use crate::config::GrafanaConfig;

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
            get_grafana_version(instance);
        }
    }
}

#[::tokio::main]
async fn get_grafana_version(instance: GrafanaInstance) {
    const GRAFANA_API_PATH: &str = "/api/health";

    let response = reqwest::get(format!("{}{}", instance.url, GRAFANA_API_PATH))
        .await
        .unwrap();

    println!("Status: {}", response.status());

    let body = response.text().await;
    println!("Body: {:?}", body);
}
