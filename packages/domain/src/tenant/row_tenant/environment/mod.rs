use datetime::RowEnvironmentTime;
use device_security_posture::RowDeviceSecurityPosture;
use location::RowEnvironmentLocation;
use network_information::RowNetworkInformation;
use risk_signals::RowRiskSignals;

pub mod datetime;
pub mod device_security_posture;
pub mod location;
pub mod network_information;
pub mod risk_signals;

#[derive(Debug, Clone)]
pub struct RowEnvironment {
    pub time: Option<RowEnvironmentTime>,
    pub location: Option<RowEnvironmentLocation>,
    pub device: Option<RowDeviceSecurityPosture>,
    pub network: Option<RowNetworkInformation>,
    pub risk: Option<RowRiskSignals>,
}
