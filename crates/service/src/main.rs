use std::ffi::OsString;
use std::sync::mpsc;
use std::time::Duration;
use windows_service::service::{
    ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus, ServiceType,
};
use windows_service::service_control_handler::{self, ServiceControlHandlerResult};
use windows_service::{define_windows_service, service_dispatcher};

const SERVICE_NAME: &str = "AdBlockService";
const SERVICE_TYPE: ServiceType = ServiceType::OWN_PROCESS;

fn run_service_loop() {
    let (shutdown_tx, shutdown_rx) = mpsc::channel();

    let event_handler = move |control_event| -> ServiceControlHandlerResult {
        match control_event {
            ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
            ServiceControl::Stop | ServiceControl::Shutdown => {
                let _ = shutdown_tx.send(());
                ServiceControlHandlerResult::NoError
            }
            _ => ServiceControlHandlerResult::NotImplemented,
        }
    };

    let status_handle = match service_control_handler::register(SERVICE_NAME, event_handler) {
        Ok(handle) => handle,
        Err(err) => {
            eprintln!("failed to register service control handler: {err}");
            return;
        }
    };

    if let Err(err) = status_handle.set_service_status(ServiceStatus {
        service_type: SERVICE_TYPE,
        current_state: ServiceState::Running,
        controls_accepted: ServiceControlAccept::STOP | ServiceControlAccept::SHUTDOWN,
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    }) {
        eprintln!("failed to report running state: {err}");
        return;
    }

    println!("AdBlockService running in the background.");
    loop {
        match shutdown_rx.recv_timeout(Duration::from_millis(250)) {
            Ok(_) | Err(mpsc::RecvTimeoutError::Disconnected) => break,
            Err(mpsc::RecvTimeoutError::Timeout) => (),
        }
    }

    if let Err(err) = status_handle.set_service_status(ServiceStatus {
        service_type: SERVICE_TYPE,
        current_state: ServiceState::Stopped,
        controls_accepted: ServiceControlAccept::empty(),
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    }) {
        eprintln!("failed to report stopped state: {err}");
    }

    println!("AdBlockService is stopping gracefully.");
}

define_windows_service!(ffi_service_main, service_main);

fn service_main(_arguments: Vec<OsString>) {
    run_service_loop();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1].eq_ignore_ascii_case("--service") {
        if let Err(err) = service_dispatcher::start(SERVICE_NAME, ffi_service_main) {
            eprintln!("failed to start service dispatcher: {err}");
            std::process::exit(1);
        }
        return;
    }

    println!("AdBlockService starting in foreground mode.");
    println!("This binary can also be launched as a Windows service via --service.");
    run_service_loop();
}
