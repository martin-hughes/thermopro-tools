use device_controller::peripheral::command::{Command, Decoded as CmdDecoded};
use device_controller::peripheral::notification::{Decoded as NtfyDecoded, Notification};

pub trait GetName {
    fn get_name(&self) -> &'static str;
}

impl GetName for Notification {
    fn get_name(&self) -> &'static str {
        match self.decoded {
            NtfyDecoded::Unknown => "Unknown",
            NtfyDecoded::Startup => "Startup",
            NtfyDecoded::SetTempMode => "Set Temp Mode",
            NtfyDecoded::ReportProbeProfile(_) => "Report Probe Profile",
            NtfyDecoded::Temperatures(_) => "Temperatures",
            NtfyDecoded::SetProbeProfile => "Set Probe Profile",
            NtfyDecoded::Error => "Error",
        }
    }
}

impl GetName for Command {
    fn get_name(&self) -> &'static str {
        match self.decoded {
            CmdDecoded::Startup => "Startup",
            CmdDecoded::SetTempMode(_) => "Set Temp Mode",
            CmdDecoded::ReportProfile(_) => "Report Probe Profile",
            CmdDecoded::SetProbeProfile(_, _) => "Set Probe Profile",
        }
    }
}
