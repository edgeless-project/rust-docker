use std;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
//Labels, HostConfig
pub struct Container {
    pub Id: String,
    pub Image: String,
    pub Status: String,
    pub Command: String,
    pub Created: u64,
    pub Names: Vec<String>,
    pub Ports: Vec<Port>,
    pub SizeRw: Option<u64>, // I guess it is optional on Mac.
    pub SizeRootFs: u64,
    pub Labels: Option<HashMap<String, String>>,
    pub HostConfig: HostConfig,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Port {
    pub IP: Option<String>,
    pub PrivatePort: u64,
    pub PublicPort: Option<u64>,
    pub Type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct HostConfig {
    pub NetworkMode: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ContainerInfo {
    pub AppArmorProfile: String,
    pub Args: Vec<String>,
    // Config
    pub Created: String,
    pub Driver: String,
    pub ExecDriver: String,
    // ExecIDs
    // HostConfig
    pub HostnamePath: String,
    pub HostsPath: String,
    pub LogPath: String,
    pub Id: String,
    pub Image: String,
    pub MountLabel: String,
    pub Name: String,
    // NetworkSettings
    pub Path: String,
    pub ProcessLabel: String,
    pub ResolvConfPath: String,
    pub RestartCount: u64,
    // State
    pub Volumes: HashMap<String, String>,
    pub VolumesRW: HashMap<String, bool>,
}

impl Clone for Container {
    fn clone(&self) -> Self {
        let container = Container {
            Id: self.Id.clone(),
            Image: self.Image.clone(),
            Status: self.Status.clone(),
            Command: self.Command.clone(),
            Created: self.Created.clone(),
            Names: self.Names.clone(),
            Ports: self.Ports.clone(),
            SizeRw: self.SizeRw,
            SizeRootFs: self.SizeRootFs,
            Labels: self.Labels.clone(),
            HostConfig: self.HostConfig.clone(),
        };

        return container;
    }
}

impl std::fmt::Display for Container {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.Id)
    }
}

impl std::clone::Clone for Port {
    fn clone(&self) -> Self {
        let port = Port {
            IP: self.IP.clone(),
            PrivatePort: self.PrivatePort.clone(),
            PublicPort: self.PublicPort.clone(),
            Type: self.Type.clone(),
        };
        return port;
    }
}

impl Clone for HostConfig {
    fn clone(&self) -> Self {
        let host_config = HostConfig {
            NetworkMode: self.NetworkMode.clone(),
        };
        return host_config;
    }
}

impl Clone for ContainerInfo {
    fn clone(&self) -> Self {
        let container_info = ContainerInfo {
            AppArmorProfile: self.AppArmorProfile.clone(),
            Args: self.Args.clone(),
            // Config
            Created: self.Created.clone(),
            Driver: self.Driver.clone(),
            ExecDriver: self.ExecDriver.clone(),
            // ExecIDs
            // HostConfig
            HostnamePath: self.HostnamePath.clone(),
            HostsPath: self.HostsPath.clone(),
            LogPath: self.LogPath.clone(),
            Id: self.Id.clone(),
            Image: self.Image.clone(),
            MountLabel: self.MountLabel.clone(),
            Name: self.Name.clone(),
            // NetworkSettings
            Path: self.Path.clone(),
            ProcessLabel: self.ProcessLabel.clone(),
            ResolvConfPath: self.ResolvConfPath.clone(),
            RestartCount: self.RestartCount,
            // State
            Volumes: self.Volumes.clone(),
            VolumesRW: self.VolumesRW.clone(),
        };
        return container_info;
    }
}

impl std::fmt::Display for ContainerInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.Id)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct PortBinding {
    pub HostIp: Option<String>,
    pub HostPort: String,
}

impl Clone for PortBinding {
    fn clone(&self) -> Self {
        PortBinding {
            HostIp: self.HostIp.clone(),
            HostPort: self.HostPort.clone(),
        }
    }
}

impl std::fmt::Display for PortBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.HostPort)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct DeviceStruct{
    pub CgroupPermissions: String,
    pub PathInContainer: String,
    pub PathOnHost: String
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct HostConfigCreate {
    pub NetworkMode: Option<String>,
    pub PublishAllPorts: Option<bool>,
    pub PortBindings: Option<HashMap<String, Vec<PortBinding>>>,
    pub Devices: Option<Vec<DeviceStruct>>
}

impl Clone for HostConfigCreate {
    fn clone(&self) -> Self {
        HostConfigCreate {
            NetworkMode: self.NetworkMode.clone(),
            PublishAllPorts: self.PublishAllPorts.clone(),
            PortBindings: self.PortBindings.clone(),
            Devices: self.Devices.clone(),
        }
    }
}

impl std::fmt::Display for HostConfigCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{:#?}", self.NetworkMode)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ContainerCreate {
    pub Image: String,
    pub Labels: Option<HashMap<String, String>>,
    pub ExposedPorts: Option<HashMap<String, HashMap<i32, i32>>>,
    pub HostConfig: Option<HostConfigCreate>,
}

impl Clone for ContainerCreate {
    fn clone(&self) -> Self {
        ContainerCreate {
            Image: self.Image.clone(),
            Labels: self.Labels.clone(),
            ExposedPorts: self.ExposedPorts.clone(),
            HostConfig: self.HostConfig.clone(),
        }
    }
}

impl std::fmt::Display for ContainerCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.Image)
    }
}
