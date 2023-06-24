use std::{env, io};
use tracing::{error, info};
use treaty::{service::Service, settings::Settings};

mod defaults;

fn main() {
    /*

    example:
    // you can also directly instatiate and plug in settings, this is intended for running tests
    let settings = settings::new(root_dir, filename)
    let service = service::new(root_dir, settings struct)

    // or just call service.start() and this will init based on settings
    let _ = service.start_user_service(TransportType::Grpc);

    // this will:
    //  - create a new dbi based on self.settings
    //  - create a new remote db based on self.settings
    //  - create a new user service handler and pass the above objects to it
    //  - start the transport layer based on the enum (Grpc, Http, Websock, Postgres, Mysql)

    */

    let version_message = format!("treaty version {}.", defaults::VERSION);
    println!("{version_message}");

    init_logs_to_screen();

    let result = Settings::new(&get_current_directory(), "Settings.toml");
    match result {
        Ok(settings) => {
            let service = Service::new(&get_current_directory(), &settings);
            service.warn_init_host_info();
            service.init_db();
            let result = service.start_services();

            match result {
                Ok(shutdown) => {
                    let mut input = String::from("");
                    println!("treaty is running. please press 'q' and enter to quit.");

                    loop {
                        io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read line");

                        if input.contains('q') {
                            info!("shutting down...");
                            shutdown.all();
                            break;
                        }
                    }

                    println!("treaty is exiting. i remain obediently yours.");
                }
                Err(e) => {
                    error!("{e:?}");
                }
            }
        }
        Err(e) => println!("{e:?}"),
    };
}

fn get_current_directory() -> String {
    let wd = env::current_dir().unwrap_or(env::temp_dir());
    let cwd = wd.to_str().unwrap().to_string();
    cwd
}

fn init_logs_to_screen() {
    use tracing_subscriber::util::SubscriberInitExt;
    use tracing_subscriber::EnvFilter;

    let filter = EnvFilter::builder().parse_lossy("treaty=trace");

    println!("{filter:?}");

    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .with_env_filter(filter)
        .finish();

    subscriber.init();
}
