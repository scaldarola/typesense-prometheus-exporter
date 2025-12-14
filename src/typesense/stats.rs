use crate::{
    cli::{CliArgs, TypesenseTarget},
    typesense::models::typesense_stats_model::TypesenseStats,
};

pub async fn get_typesense_stats(
    client: &reqwest::Client,
    args: &CliArgs,
    target: &TypesenseTarget,
) -> Result<TypesenseStats, reqwest::Error> {
    let url = format!(
        "{}://{}:{}/stats.json",
        args.typesense_protocol, target.host, target.port
    );

    client
        .get(url)
        .header("X-TYPESENSE-API-KEY", &args.typesense_api_key)
        .send()
        .await?
        .error_for_status()?
        .json::<TypesenseStats>()
        .await
}
