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
        Container {
            Id: self.Id.clone(),
            Image: self.Image.clone(),
            Status: self.Status.clone(),
            Command: self.Command.clone(),
            Created: self.Created,
            Names: self.Names.clone(),
            Ports: self.Ports.clone(),
            SizeRw: self.SizeRw,
            SizeRootFs: self.SizeRootFs,
            Labels: self.Labels.clone(),
            HostConfig: self.HostConfig.clone(),
        }
    }
}

impl std::fmt::Display for Container {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.Id)
    }
}

impl std::clone::Clone for Port {
    fn clone(&self) -> Self {
        Port {
            IP: self.IP.clone(),
            PrivatePort: self.PrivatePort,
            PublicPort: self.PublicPort,
            Type: self.Type.clone(),
        }
    }
}

impl Clone for HostConfig {
    fn clone(&self) -> Self {
        HostConfig {
            NetworkMode: self.NetworkMode.clone(),
        }
    }
}

impl Clone for ContainerInfo {
    fn clone(&self) -> Self {
        ContainerInfo {
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
        }
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

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct HostConfigCreate {
    pub NetworkMode: Option<String>,
    pub PublishAllPorts: Option<bool>,
    pub PortBindings: Option<HashMap<String, Vec<PortBinding>>>,
    pub AutoRemove: Option<bool>,
    pub Binds: Option<Vec<String>>,
}

impl Clone for HostConfigCreate {
    fn clone(&self) -> Self {
        HostConfigCreate {
            NetworkMode: self.NetworkMode.clone(),
            PublishAllPorts: self.PublishAllPorts,
            PortBindings: self.PortBindings.clone(),
            AutoRemove: self.AutoRemove.clone(),
            Binds: self.Binds.clone(),
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
    pub Hostname: Option<String>,
    pub Domainname: Option<String>,
    pub User: Option<String>,
    pub AttachStdin: Option<bool>,
    pub AttachStdout: Option<bool>,
    pub AttachStderr: Option<bool>,
    pub ExposedPorts: Option<HashMap<String, EmptyObject>>,
    pub Tty: Option<bool>,
    pub OpenStdin: Option<bool>,
    pub StdinOnce: Option<bool>,
    pub Env: Option<Vec<String>>,
    pub Cmd: Option<Vec<String>>,
    pub ArgsEscaped: Option<bool>,
    pub Image: String,
    pub Volumes: Option<HashMap<String, EmptyObject>>,
    pub WorkingDir: Option<String>,
    pub Entrypoint: Option<Vec<String>>,
    pub NetworkDisabled: Option<bool>,
    pub MacAddress: Option<String>,
    pub OnBuild: Option<Vec<String>>,
    pub Labels: Option<HashMap<String, String>>,
    pub StopSignal: Option<String>,
    pub StopTimeout: Option<u64>,
    pub Shell: Option<Vec<String>>,
    pub HostConfig: Option<HostConfigCreate>,
}

impl Clone for ContainerCreate {
    fn clone(&self) -> Self {
        ContainerCreate {
            Hostname: self.Hostname.clone(),
            Domainname: self.Domainname.clone(),
            User: self.User.clone(),
            AttachStdin: self.AttachStdin,
            AttachStdout: self.AttachStdout,
            AttachStderr: self.AttachStderr,
            ExposedPorts: self.ExposedPorts.clone(),
            Tty: self.Tty,
            OpenStdin: self.OpenStdin,
            StdinOnce: self.StdinOnce,
            Env: self.Env.clone(),
            Cmd: self.Cmd.clone(),
            ArgsEscaped: self.ArgsEscaped,
            Image: self.Image.clone(),
            Volumes: self.Volumes.clone(),
            WorkingDir: self.WorkingDir.clone(),
            Entrypoint: self.Entrypoint.clone(),
            NetworkDisabled: self.NetworkDisabled,
            MacAddress: self.MacAddress.clone(),
            OnBuild: self.OnBuild.clone(),
            Labels: self.Labels.clone(),
            StopSignal: self.StopSignal.clone(),
            StopTimeout: self.StopTimeout,
            Shell: self.Shell.clone(),
            HostConfig: self.HostConfig.clone(),
        }
    }
}

impl std::fmt::Display for ContainerCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.Image)
    }
}

impl Default for ContainerCreate {
    fn default() -> Self {
        ContainerCreate {
            Hostname: None,
            Domainname: None,
            User: None,
            AttachStdin: None,
            AttachStdout: None,
            AttachStderr: None,
            ExposedPorts: None,
            Tty: None,
            OpenStdin: None,
            StdinOnce: None,
            Env: None,
            Cmd: None,
            ArgsEscaped: None,
            Image: String::new(),
            Volumes: None,
            WorkingDir: None,
            Entrypoint: None,
            NetworkDisabled: None,
            MacAddress: None,
            OnBuild: None,
            Labels: None,
            StopSignal: None,
            StopTimeout: None,
            Shell: None,
            HostConfig: None,
        }
    }
}

/// A struct representing the value `{}` in JSON.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct EmptyObject {}
