use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Network {
    pub Name: String,
    pub Id: String,
    pub Created: String,
    pub Scope: String,
    pub Driver: Option<String>,
    pub EnableIPv6: bool,
    pub Internal: bool,
    pub Attachable: bool,
    pub Ingress: bool,
    pub Options: HashMap<String, String>,
    pub Labels: Option<HashMap<String, String>>, //IPAM
}

impl Clone for Network {
    fn clone(&self) -> Self {
        Network {
            Name: self.Name.clone(),
            Id: self.Id.clone(),
            Created: self.Created.clone(),
            Scope: self.Scope.clone(),
            Driver: self.Driver.clone(),
            EnableIPv6: self.EnableIPv6,
            Internal: self.Internal,
            Attachable: self.Attachable,
            Ingress: self.Ingress,
            Options: self.Options.clone(),
            Labels: self.Labels.clone(),
        }
    }
}

impl std::fmt::Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.Id)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct NetworkCreate {
    pub Name: String,
    pub CheckDuplicate: Option<bool>,
    pub Driver: Option<String>,
    pub Internal: Option<bool>,
    pub Attachable: Option<bool>,
    pub Ingress: Option<bool>,
    pub EnableIPv6: Option<bool>,
    pub Options: Option<HashMap<String, String>>,
    pub Labels: Option<HashMap<String, String>>, //IPAM
}

impl Clone for NetworkCreate {
    fn clone(&self) -> Self {
        NetworkCreate {
            Name: self.Name.clone(),
            CheckDuplicate: self.CheckDuplicate,
            Driver: self.Driver.clone(),
            Internal: self.Internal,
            Attachable: self.Attachable,
            Ingress: self.Ingress,
            EnableIPv6: self.EnableIPv6,
            Options: self.Options.clone(),
            Labels: self.Labels.clone(),
        }
    }
}

impl std::fmt::Display for NetworkCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.Name)
    }
}
