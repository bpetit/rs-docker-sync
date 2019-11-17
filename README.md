# This is a fork

This crate and repo were forked from [https://github.com/ghmlee/rust-docker](https://github.com/ghmlee/rust-docker) because the original repo seems to no longer be maintained.

With this in mind I will try to keep this crate as backwards compatible as possible.

Issues and PRs welcome.

# Docker

[![Build Status](https://gitlab.com/kblobr/rust-docker/badges/master/build.svg)](https://gitlab.com/kblobr/rust-docker)

This is a Docker Remote API binding in Rust. Documentation is available [here](https://docs.rs/rs-docker).

## Quick start

```
[dependencies]
rs-docker = "0.0.58"
```

```rust
extern crate rs_docker;

use rs_docker::Docker;

fn main() {
    let docker = match Docker::connect("unix:///var/run/docker.sock") {
    	Ok(docker) => docker,
        Err(e) => { panic!("{}", e); }
    };
}
```

## Debug
* Rust (>= v1.4.0)
* Docker (>= v1.5.0)

## Examples

### Networks

```rust
extern crate rs_docker;

use rs_docker::Docker;

fn main() {
    let mut docker = match Docker::connect("unix:///var/run/docker.sock") {
    	Ok(docker) => docker,
        Err(e) => { panic!("{}", e); }
    };

    let networks = match docker.get_networks() {
        Ok(networks) => networks,
        Err(e) => { panic!("{}", e); }
    };
}
```


### Containers

```rust
extern crate rs_docker;

use rs_docker::Docker;

fn main() {
    let mut docker = match Docker::connect("unix:///var/run/docker.sock") {
    	Ok(docker) => docker,
        Err(e) => { panic!("{}", e); }
    };

    let containers = match docker.get_containers(false) {
        Ok(containers) => containers,
        Err(e) => { panic!("{}", e); }
    };
}
```

### Stats

```rust
extern crate rs_docker;

use rs_docker::Docker;

fn main() {
    let mut docker = match Docker::connect("unix:///var/run/docker.sock") {
    	Ok(docker) => docker,
        Err(e) => { panic!("{}", e); }
    };

    let containers = match docker.get_containers(false) {
        Ok(containers) => containers,
        Err(e) => { panic!("{}", e); }
    };

    let stats = match docker.get_stats(&containers[0]) {
        Ok(stats) => stats,
        Err(e) => { panic!("{}", e); }
    };
}
```

### Images

```rust
extern crate rs_docker;

use rs_docker::Docker;

fn main() {
    let mut docker = match Docker::connect("unix:///var/run/docker.sock") {
    	Ok(docker) => docker,
        Err(e) => { panic!("{}", e); }
    };

    let images = match docker.get_images(false) {
        Ok(images) => images,
        Err(e) => { panic!({}, e); }
    };
}

```

### Info

```rust
extern crate rs_docker;

use rs_docker::Docker;

fn main() {
    let mut docker = match Docker::connect("unix:///var/run/docker.sock") {
    	Ok(docker) => docker,
        Err(e) => { panic!("{}", e); }
    };

    let info = match docker.get_system_info() {
        Ok(info) => info,
        Err(e) => { panic!("{}", e); }
    };
}
```

### Processes

```rust
extern crate rs_docker;

use rs_docker::Docker;

fn main() {
    let mut docker = match Docker::connect("unix:///var/run/docker.sock") {
    	Ok(docker) => docker,
        Err(e) => { panic!("{}", e); }
    };

    let containers = match docker.get_containers(false) {
        Ok(containers) => containers,
        Err(e) => { panic!("{}", e); }
    };

    let processes = match docker.get_processes(&containers[0]) {
        Ok(processes) => processes,
        Err(e) => { panic!("{}", e); }
    };
}
```

### Filesystem changes

```rust
extern crate rs_docker;

use rs_docker::Docker;

fn main() {
    let mut docker = match Docker::connect("unix:///var/run/docker.sock") {
    	Ok(docker) => docker,
        Err(e) => { panic!("{}", e); }
    };

    let containers = match docker.get_containers(false) {
        Ok(containers) => containers,
        Err(e) => { panic!("{}", e); }
    };

    let changes = match docker.get_filesystem_changes(&containers[0]) {
        Ok(changes) => changes,
        Err(e) => { panic!("{}", e); }
    };
}
```

### Export a container

```rust
extern crate rs_docker;

use rs_docker::Docker;

fn main() {
    let mut docker = match Docker::connect("unix:///var/run/docker.sock") {
    	Ok(docker) => docker,
        Err(e) => { panic!("{}", e); }
    };

    let containers = match docker.get_containers(false) {
        Ok(containers) => containers,
        Err(e) => { panic!("{}", e); }
    };

    let bytes = match docker.export_container(&containers[0]) {
        Ok(bytes) => bytes,
        Err(e) => { panic!("{}", e); }
    };
}
```

### Create an image

```rust
extern crate rs_docker;

use rs_docker::Docker;

fn main() {
    let mut docker = match Docker::connect("unix:///var/run/docker.sock") {
    	Ok(docker) => docker,
        Err(e) => { panic!("{}", e); }
    };

    let image = "debian".to_string();
    let tag = "latest".to_string();
    
    let statuses = match docker.create_image(image, tag) {
        Ok(statuses) => statuses,
        Err(e) => { panic!("{}", e); }
    };
    
    match statuses.last() {
        Some(last) => {
            println!("{}", last.clone().status.unwrap());
        }
        None => { println!("none"); }
    }
}
```

### Ping the docker server

```rust
extern crate rs_docker;

use rs_docker::Docker;

fn main() {
    let mut docker = match Docker::connect("unix:///var/run/docker.sock") {
    	Ok(docker) => docker,
        Err(e) => { panic!("{}", e); }
    };
    
    let ping = match docker.ping() {
        Ok(ping) => ping,
        Err(e) => { panic!("{}", e); }
    };
}
```

### Show the docker version information

```rust
extern crate rs_docker;

use rs_docker::Docker;

fn main() {
    let mut docker = match Docker::connect("unix:///var/run/docker.sock") {
    	Ok(docker) => docker,
        Err(e) => { panic!("{}", e); }
    };
    
    let version = match docker.get_version() {
        Ok(version) => version,
        Err(e) => {panic!("{}",e)}
    };
}
```

## Docker Toolbox

By default, `Docker Toolbox` runs `docker` with TLS enabled. It auto-generates certificates. The `docker-machine` will copy them to `~/.docker/machine/certs` on the host machine once the VM has started.

### Example

```rust
extern crate rs_docker;

use rs_docker::Docker;
use std::path::Path;

fn main() {
    let key = Path::new("/Users/<username>/.docker/machine/certs/key.pem");
    let cert = Path::new("/Users/<username>/.docker/machine/certs/cert.pem");
    let ca = Path::new("/Users/<username>/.docker/machine/certs/ca.pem");

    let mut docker = match Docker::connect("tcp://192.168.99.100:2376") {
    	Ok(docker) => docker,
        Err(e) => { panic!("{}", e); }
    };
    docker.set_tls(&key, &cert, &ca).unwrap();
}
```

## Contributing

To have a consistent dev environment one can use the docker image in /devenv like so:
1. `git clone https://gitlab.com/kblobr/rust-docker`
2. `cd rust-docker/devenv`
3. `./build_docker` (this assumes your user can run docker commands, otherwise `sudo`)
4. `./run_docker -ti`
5. Already inside the container:
  1. `cd Code`
  2. `cargo test`

For changes:

1. Fork it
2. Create your a new remote upstream repository (`git remote add upstream https://gitlab.com/kblobr/rust-docker`)
3. Commit your changes (`git commit -m 'Adds some feature'`)
4. Push to the branch (`git push origin your-branch`)
5. Create new Pull Request
