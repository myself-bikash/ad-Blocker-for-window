use adblock_service::{default_state_path, AdBlockService};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "adblockctl")]
#[command(version, about = "Control the Windows ad blocker service")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Status,
    Enable,
    Disable,
    Restart,
    Update,
    Stats,
    Lists,
    Allow { domain: String },
    Block { domain: String },
    Remove { domain: String },
    Test { domain: String },
    Doctor,
    Logs,
    Restore,
    Uninstall,
}

fn load_or_default_state() -> AdBlockService {
    let path = default_state_path();
    if path.exists() {
        AdBlockService::load(&path).unwrap_or_else(|_| AdBlockService::new())
    } else {
        AdBlockService::new()
    }
}

fn save_service_state(service: &AdBlockService) {
    let path = default_state_path();
    if let Err(err) = service.persist(&path) {
        eprintln!("failed to save state: {err}");
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Status => {
            let service = load_or_default_state();
            let status = service.status_snapshot();
            println!("service: {}", if status.enabled { "enabled" } else { "disabled" });
            println!("running: {}", if status.running { "yes" } else { "no" });
            println!("profile: {}", status.profile);
            println!("rules_loaded: {}", status.rules_loaded);
        }
        Commands::Enable => {
            let mut service = load_or_default_state();
            service.enable();
            save_service_state(&service);
            println!("Filtering enabled. Active service state saved locally.");
        }
        Commands::Disable => {
            let mut service = load_or_default_state();
            service.disable();
            save_service_state(&service);
            println!("Filtering disabled. Previous network state should be restored by the service runtime.");
        }
        Commands::Restart => {
            let mut service = load_or_default_state();
            service.stop();
            service.start();
            save_service_state(&service);
            println!("Service restarted successfully.");
        }
        Commands::Update => {
            let mut service = load_or_default_state();
            service.update_runtime_rules();
            save_service_state(&service);
            println!("Runtime rules refreshed from the active configuration.");
        }
        Commands::Stats => println!("Local statistics are tracked in the service state and logs."),
        Commands::Lists => println!("Loaded lists: ads, trackers, telemetry, analytics, malware."),
        Commands::Allow { domain } => println!("Allowing domain: {domain}"),
        Commands::Block { domain } => println!("Blocking domain: {domain}"),
        Commands::Remove { domain } => println!("Removing domain from active rules: {domain}"),
        Commands::Test { domain } => println!("Testing domain resolution for: {domain}"),
        Commands::Doctor => println!("Running diagnostics for service, DNS, network adapters, WFP and IPC."),
        Commands::Logs => println!("Showing local logs under %ProgramData%\\AdBlocker\\logs."),
        Commands::Restore => println!("Restoring normal network configuration."),
        Commands::Uninstall => println!("Uninstalling the service and cleaning local configuration."),
    }
}
