use appflowy_cloud::application::{init_state, Application};
use appflowy_cloud::config::config::get_configuration;
use appflowy_cloud::telemetry::init_subscriber;
use tracing::info;

// https://github.com/polarsignals/rust-jemalloc-pprof?tab=readme-ov-file#usage
#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[allow(non_upper_case_globals)]
#[export_name = "malloc_conf"]
pub static malloc_conf: &[u8] = b"prof:true,prof_active:true,lg_prof_sample:19\0";

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
  let level = std::env::var("RUST_LOG").unwrap_or("info".to_string());
  println!("AppFlowy Cloud with RUST_LOG={}", level);
  let mut filters = vec![];
  filters.push(format!("actix_web={}", level));
  filters.push(format!("collab={}", level));
  filters.push(format!("collab_sync={}", level));
  filters.push(format!("appflowy_cloud={}", level));
  filters.push(format!("collab_plugins={}", level));
  filters.push(format!("realtime={}", level));
  filters.push(format!("database={}", level));
  filters.push(format!("storage={}", level));
  filters.push(format!("gotrue={}", level));
  let conf =
    get_configuration().map_err(|e| anyhow::anyhow!("Failed to read configuration: {}", e))?;
  init_subscriber(&conf.app_env, filters);

  // If current build is debug and the feature "custom_env" is not enabled, load from .env
  // otherwise, load from .env.without_nginx.
  if cfg!(debug_assertions) {
    #[cfg(not(feature = "custom_env"))]
    {
      info!("custom_env is disable, load from .env");
      dotenvy::dotenv().ok();
    }

    #[cfg(feature = "custom_env")]
    {
      match dotenvy::from_filename(".env.without_nginx") {
        Ok(_) => {
          info!("custom_env is enabled, load from .env.without_nginx");
        },
        Err(err) => {
          tracing::error!(
            "Failed to load .env.without_nginx: {}, fallback to .env file",
            err
          );
          dotenvy::dotenv().ok();
        },
      }
    }
  } else {
    // In release, always load from .env
    dotenvy::dotenv().ok();
  }

  let state = init_state(&conf)
    .await
    .map_err(|e| anyhow::anyhow!("Failed to initialize application state: {}", e))?;
  let application = Application::build(conf, state).await?;
  application.run_until_stopped().await?;

  Ok(())
}
