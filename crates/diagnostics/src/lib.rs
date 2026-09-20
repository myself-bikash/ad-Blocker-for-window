use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DiagnosticsReport {
    pub service_status: bool,
    pub dns_status: bool,
    pub network_status: bool,
    pub wfp_status: bool,
    pub ipc_status: bool,
}

impl DiagnosticsReport {
    pub fn healthy(&self) -> bool {
        self.service_status && self.dns_status && self.network_status && self.wfp_status && self.ipc_status
    }
}

pub fn doctor_summary() -> &'static str {
    "Diagnostics include service health, DNS validation, network adapter checks, WFP readiness, and IPC connectivity."
}
