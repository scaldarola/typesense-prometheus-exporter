use crate::{
    cli::{CliArgs, TypesenseTarget},
    typesense::models::typesense_metrics_model::TypesenseMetrics,
};

pub async fn get_typesense_metrics(
    client: &reqwest::Client,
    args: &CliArgs,
    target: &TypesenseTarget,
) -> Result<TypesenseMetrics, reqwest::Error> {
    let url: String = format!(
        "{}://{}:{}/metrics.json",
        args.typesense_protocol, target.host, target.port
    );

    client
        .get(url)
        .header("X-TYPESENSE-API-KEY", &args.typesense_api_key)
        .send()
        .await?
        .error_for_status()?
        .json::<TypesenseMetrics>()
        .await
}
