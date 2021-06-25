use crate::container::{Container, ContainerInfo};
use crate::filesystem::FilesystemChange;
use crate::image::Image;
use crate::network::{Network, NetworkCreate};
use crate::process::{Process, Top};
use crate::stats::Stats;
use crate::system::SystemInfo;
use crate::version::Version;
use crate::event::Event;
use http::method::Method;
use isahc::{config::Dialer, prelude::*, send, Body, Request};
use std::io::Read;

pub struct Docker {
    dialer: Dialer,
}

impl Docker {
    pub fn connect() -> std::io::Result<Docker> {
        let path = String::from("/var/run/docker.sock");
        let dialer = Dialer::unix_socket(path);

        Ok(Docker { dialer })
    }

    fn request(&self, method: Method, url: &str, body: String) -> String {
        #[cfg(unix)]
        let req = Request::builder()
            .uri(format!("http://localhost{}", url))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .method(method)
            .dial(self.dialer.clone())
            .body(Body::from(body))
            .expect("failed to build request");
        let mut res = send(req).unwrap();

        if !res.status().is_success() {
            panic!("Request failed");
        }

        let body = res.body_mut();
        let mut buf = String::new();
        if let Err(e) = body.read_to_string(&mut buf) {
            panic!("{}", e);
        }
        buf
    }

    //
    // Networks
    //

    pub fn get_networks(&mut self) -> std::io::Result<Vec<Network>> {
        let body = self.request(Method::GET, "/networks", "".to_string());

        match serde_json::from_str(&body) {
            Ok(networks) => Ok(networks),
            Err(e) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                e.to_string(),
            )),
        }
    }

    pub fn create_network(&mut self, network: NetworkCreate) -> std::io::Result<String> {
        let body = self.request(
            Method::POST,
            "/networks/create",
            serde_json::to_string(&network).unwrap(),
        );

        let status: serde_json::Value = match serde_json::from_str(&body) {
            Ok(status) => status,
            Err(e) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    e.to_string(),
                ));
            }
        };
        match status.get("Id") {
            Some(id) => Ok(id.as_str().unwrap().to_string()),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                status.get("message").unwrap().to_string(),
            )),
        }
    }

    pub fn delete_network(&mut self, id_or_name: &str) -> std::io::Result<String> {
        let body = self.request(
            Method::DELETE,
            &format!("/networks/{}", id_or_name),
            "".to_string(),
        );

        match serde_json::from_str::<serde_json::Value>(&body) {
            Ok(status) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                status["message"].to_string(),
            )),
            Err(_e) => Ok("".to_string()),
        }
    }

    //
    // Containers
    //

    pub fn get_containers(&mut self, all: bool) -> std::io::Result<Vec<Container>> {
        let a = match all {
            true => "1",
            false => "0",
        };

        let body = self.request(
            Method::GET,
            &format!("/containers/json?all={}&size=1", a),
            "".to_string(),
        );

        match serde_json::from_str(&body) {
            Ok(containers) => Ok(containers),
            Err(e) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                e.to_string(),
            )),
        }
    }

    pub fn get_processes(&mut self, container: &Container) -> std::io::Result<Vec<Process>> {
        let body = self.request(
            Method::GET,
            &format!("/containers/{}/top", container.Id),
            "".to_string(),
        );

        let top: Top = match serde_json::from_str(&body) {
            Ok(top) => top,
            Err(e) => {
                let err = std::io::Error::new(std::io::ErrorKind::InvalidInput, e.to_string());
                return Err(err);
            }
        };

        let mut processes: Vec<Process> = Vec::new();
        let process_iter = top.Processes.iter();

        for process in process_iter {
            let mut p = Process {
                user: String::new(),
                pid: String::new(),
                cpu: None,
                memory: None,
                vsz: None,
                rss: None,
                tty: None,
                stat: None,
                start: None,
                time: None,
                command: String::new(),
            };

            let i: usize = 0;
            let value_iter = process.iter();
            for value in value_iter {
                let key = &top.Titles[i];
                match key.as_ref() {
                    "USER" => p.user = value.clone(),
                    "PID" => p.pid = value.clone(),
                    "%CPU" => p.cpu = Some(value.clone()),
                    "%MEM" => p.memory = Some(value.clone()),
                    "VSZ" => p.vsz = Some(value.clone()),
                    "RSS" => p.rss = Some(value.clone()),
                    "TTY" => p.tty = Some(value.clone()),
                    "STAT" => p.stat = Some(value.clone()),
                    "START" => p.start = Some(value.clone()),
                    "TIME" => p.time = Some(value.clone()),
                    "COMMAND" => p.command = value.clone(),
                    _ => {}
                }
            }

            processes.push(p);
        }

        Ok(processes)
    }

    pub fn get_stats(&mut self, container: &Container) -> std::io::Result<Stats> {
        if !container.Status.contains("Up") {
            let err = std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "The container is already stopped.",
            );
            return Err(err);
        }

        let body = self.request(
            Method::GET,
            &format!("/containers/{}/stats", container.Id),
            "".to_string(),
        );

        match serde_json::from_str(&body) {
            Ok(stats) => Ok(stats),
            Err(e) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                e.to_string(),
            )),
        }
    }

    //
    // Image
    //

    pub fn get_images(&mut self, all: bool) -> std::io::Result<Vec<Image>> {
        let a = match all {
            true => "1",
            false => "0",
        };
        let body = self.request(
            Method::GET,
            &format!("/images/json?all={}", a),
            "".to_string(),
        );

        match serde_json::from_str(&body) {
            Ok(images) => Ok(images),
            Err(e) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                e.to_string(),
            )),
        }
    }

    pub fn get_system_info(&mut self) -> std::io::Result<SystemInfo> {
        let body = self.request(Method::GET, "/info", "".to_string());

        match serde_json::from_str(&body) {
            Ok(info) => Ok(info),
            Err(e) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                e.to_string(),
            )),
        }
    }

    pub fn get_container_info(&mut self, container: &Container) -> std::io::Result<ContainerInfo> {
        let body = self.request(
            Method::GET,
            &format!("/containers/{}/json", container.Id),
            "".to_string(),
        );

        match serde_json::from_str(&body) {
            Ok(body) => Ok(body),
            Err(e) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                e.to_string(),
            )),
        }
    }

    pub fn get_filesystem_changes(
        &mut self,
        container: &Container,
    ) -> std::io::Result<Vec<FilesystemChange>> {
        let body = self.request(
            Method::GET,
            &format!("/containers/{}/changes", container.Id),
            "".to_string(),
        );

        match serde_json::from_str(&body) {
            Ok(body) => Ok(body),
            Err(e) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                e.to_string(),
            )),
        }
    }

    pub fn ping(&mut self) -> std::io::Result<String> {
        Ok(self.request(Method::GET, "/_ping", "".to_string()))
    }

    pub fn get_version(&mut self) -> std::io::Result<Version> {
        let body = self.request(Method::GET, "/version", "".to_string());

        match serde_json::from_str(&body) {
            Ok(r_body) => Ok(r_body),
            Err(e) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                e.to_string(),
            )),
        }
    }

    pub fn get_events(&mut self, since: Option<String>, until: Option<String>) -> std::io::Result<Vec<Event>> {
        let mut url = "/events".to_string();
        let mut options = "".to_string();
        if let Some(since_val) = since {
            options.push_str("since=");
            options.push_str(&since_val);
        }
        if let Some(until_val) = until {
            if !options.is_empty() {
                options.push_str("&");
            }
            options.push_str("until=");
            options.push_str(&until_val);
        }
        if !options.is_empty() {
            url.push_str("?");
            url.push_str(&options);
        }
        let body = self.request(Method::GET, &url, "".to_string());
        match serde_json::from_str(&body) {
            Ok(r_body) => Ok(r_body),
            Err(e) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                e.to_string()
            ))
        }
    }
}
