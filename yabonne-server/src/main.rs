extern crate config;
#[macro_use]
extern crate crossbeam_channel;
#[macro_use]
extern crate log;
extern crate signal_hook;

use crossbeam_channel::unbounded;
use futures::future::join_all;
use signal_hook::iterator::Signals;
use tokio::signal::unix::{signal, SignalKind};
use yabonne_server::frontend::FrontEndController;
use yabonne_server::backend::BackEndController;
use yabonne_server::shared::util::rabbitmq::RabbitMQContext;
use yabonne_server::YaBoNneConfig;
use env_logger::Env;


static SETTINGS_FILE: &str = "conf/settings.toml";

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    info!("Starting Yabonne-Server");
    let yabonne_config = YaBoNneConfig::new(SETTINGS_FILE).unwrap();

    //RabbitMQ contexts
    let rabbitmq_context_frontend = RabbitMQContext::new(
        yabonne_config.frontend_rabbitmq_config.clone(),
        yabonne_config.backend_rabbitmq_config.clone(),
    ).await;
    let rabbitmq_context_backend = RabbitMQContext::new(
        yabonne_config.frontend_rabbitmq_config.clone(),
        yabonne_config.backend_rabbitmq_config.clone(),
    ).await;

    //Database contexts
    let database_config_map =
        yabonne_config.domain_database_config.clone();

    //Workflow Config
    let workflow_config = yabonne_config.camunda_workflow_config.clone();
    //create channels
    let (frontend_sending_channel, frontend_receiving_channel) = unbounded();
    let (backend_sending_channel, backend_receiving_channel) = unbounded();

    let front_end_controller = FrontEndController::new(frontend_receiving_channel);
    let back_end_controller = BackEndController::new(backend_receiving_channel);

    let redis_config_clone = yabonne_config.redis_config.clone();
    let front_end_handle = tokio::spawn(async move {
        info!("Inside the front end handle");
        front_end_controller.start(rabbitmq_context_frontend.unwrap(),
                                   redis_config_clone).await;
    });

    let back_end_handle = tokio::spawn(async move {
        info!("Inside the back end handle");
        back_end_controller.start(rabbitmq_context_backend.unwrap(),
                                  database_config_map, workflow_config).await;
    });

    let signal_blocking_handle = tokio::spawn(async move {
        info!("Waiting for Ctrl-C signal");
        /*       let stream = signal(SignalKind::hangup()).unwrap();
               let mut signals = Signals::new(&[signal_hook::consts::SIGINT]).unwrap();
               for signal in &signals {
                   match signal {
                       signal_hook::consts::SIGINT => {
                           info!("Ctrl-C signal received");
                           //signal_sender_channel.send(1);
                       }
                       _ => {}
                   }
               }*/
    });

    join_all(vec![
        front_end_handle,
        back_end_handle,
        signal_blocking_handle,
    ]).await;

    Ok(())
}
