use anyhow::Result;
use clap::Parser;
use kube::{
    api::{Api, ListParams},
    Client,
};
use k8s_openapi::api::core::v1::Pod;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Namespace to list pods from. If not specified, uses the current context's namespace
    #[arg(short, long)]
    namespace: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Create a Kubernetes client
    let client = Client::try_default().await?;
    
    // Get the pods API for the specified namespace or default namespace
    let pods: Api<Pod> = if let Some(ns) = cli.namespace {
        Api::namespaced(client, &ns)
    } else {
        eprintln!("Warning: Using default namespace. Use -n/--namespace to specify a namespace.");
        Api::default_namespaced(client)
    };

    // List pods
    let lp = ListParams::default();
    let pod_list = pods.list(&lp).await?;

    // Print pod information
    println!("NAMESPACE\tNAME\t\tSTATUS\t\tRESTARTS\tAGE");
    for pod in pod_list.items {
        let status = pod.status.as_ref().and_then(|s| s.phase.as_deref()).unwrap_or("Unknown");
        let restart_count = pod.status.as_ref().and_then(|s| s.container_statuses.as_ref())
            .map(|cs| cs.iter().map(|c| c.restart_count).sum::<i32>())
            .unwrap_or(0);
        
        let age = pod.metadata.creation_timestamp
            .map(|t| {
                let duration = chrono::Utc::now() - t.0;
                format!("{}h", duration.num_hours())
            })
            .unwrap_or_else(|| "Unknown".to_string());

        println!(
            "{}\t{}\t{}\t{}\t\t{}",
            pod.metadata.namespace.as_deref().unwrap_or("default"),
            pod.metadata.name.as_deref().unwrap_or("Unknown"),
            status,
            restart_count,
            age
        );
    }

    Ok(())
} 