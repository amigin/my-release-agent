use crate::app::AppContext;

pub async fn release(app: &AppContext, configuration_id: &str) -> Result<String, String> {
    let settings = app
        .settings_reader
        .use_settings(|settings_model| {
            settings_model
                .find_configuration(configuration_id)
                .map(|itm| {
                    (
                        itm.path.clone(),
                        itm.reset_cache_cloud_flare_api_key.clone(),
                    )
                })
        })
        .await;

    let Some((path, cloud_flare_config)) = settings else {
        return Err(format!("Configuration not found"));
    };

    let file = rust_extensions::file_utils::format_path(path);
    let mut result = String::new();

    execute_bash_command(vec!["docker-compose", "-f", file.as_str(), "pull"]).await?;
    result.push_str("Docker compose pull ok\n");

    execute_bash_command(vec!["docker-compose", "-f", file.as_str(), "up", "-d"]).await?;
    result.push_str("Docker compose up -d ok\n");

    if let Some(cloud_flare_config) = cloud_flare_config {
        let mut cloud_flare_config = cloud_flare_config.split(':');

        let api_key = cloud_flare_config.next().unwrap();
        let zone_id = cloud_flare_config.next().unwrap();
        let mut cf_purge_result = crate::scripts::purge_cf_caches(api_key, zone_id).await?;
        cf_purge_result.push('\n');
        result.push_str(cf_purge_result.as_str());
    }

    Ok(result)
}

async fn execute_bash_command(mut args: Vec<&str>) -> Result<(), String> {
    let command_str = format!("{:?}", args);
    let mut command = tokio::process::Command::new(args.remove(0));

    while args.len() > 0 {
        let arg = args.remove(0);
        command.arg(arg);
    }

    match command.output().await {
        Ok(result) => {
            println!(
                "Ok: {}. Result: {:?}",
                command_str,
                std::str::from_utf8(result.stdout.as_slice())
            );
            Ok(())
        }
        Err(err) => Err(format!("{:?}", err)),
    }
}
