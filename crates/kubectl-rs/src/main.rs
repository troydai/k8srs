use clap::Parser;
use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::{Api, ListParams},
    Client,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Namespace to list pods from. If not specified, uses the current context's namespace
    #[arg(short, long)]
    namespace: Option<String>,
}

fn get_pods_api(client: Client, namespace: Option<String>) -> anyhow::Result<Api<Pod>> {
    if let Some(ns) = namespace {
        Ok(Api::namespaced(client, &ns))
    } else {
        Ok(Api::default_namespaced(client))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let client = Client::try_default().await?;
    let pods_api = get_pods_api(client, cli.namespace)?;
    let pod_list = pods_api.list(&ListParams::default()).await?;

    for pod in pod_list.items {
        // namespace
        let ns = pod.metadata.namespace.as_deref().unwrap_or("default");

        // name
        let name = pod.metadata.name.as_deref().unwrap_or("Unknown");

        // status
        let status = pod.status.as_ref();

        // phase
        let phase = status.and_then(|s| s.phase.as_deref()).unwrap_or("Unknown");

        // restart cound
        let restart_count = status
            .and_then(|s| s.container_statuses.as_ref())
            .map(|cs| cs.iter().map(|c| c.restart_count).sum::<i32>())
            .unwrap_or(0);

        // creation
        let age = pod
            .metadata
            .creation_timestamp
            .map(|t| {
                let duration = chrono::Utc::now() - t.0;
                format!("{}h", duration.num_hours())
            })
            .unwrap_or("Unknown".to_string());

        println!("{}\t{}\t{}\t{}\t{}", ns, phase, name, restart_count, age);
    }

    Ok(())
}
