 This is a fork

This is a fork of [https://gitlab.com/kblobr/rust-docker](https://gitlab.com/kblobr/rust-docker) ([rs-docker](https://crates.io/crates/rs-docker) on crate.io) which itself is a fork from [https://github.com/ghmlee/rust-docker](https://github.com/ghmlee/rust-docker) ([docker](https://crates.io/crates/docker) on crates.io). Both repositories seemed to be no longer be maintained. The main reason for this fork, besides the maintainance, is that [Scaphandre](https://github.com/hubblo-org/scaphandre/) needed a synchronous library to talk to Docker socket.

Issues and PRs welcome.

# Docker

Minimalistic, synchronous, read-only client for local Docker socket.

Documentation is available [here](https://docs.rs/docker-sync).

## Quick start

```
[dependencies]
rs-docker-sync = "0.1.2"
```

```rust
extern crate docker_sync;

use docker_sync::Docker;

fn main() {
    let docker = match Docker::connect() { // we consider the local Docker socket by default, with the default path (/var/run/docker.sock), no need to precise the path
    	Ok(docker) => docker,
        Err(e) => { panic!("{}", e); }
    };
}
```

## Requirements

* Rust (>= v1.4.0)
* Docker (>= v1.5.0)

## Examples

### Networks

```rust
extern crate docker_sync;

use docker_sync::Docker;

fn main() {
    let mut docker = match Docker::connect() {
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
extern crate docker_sync;

use docker_sync::Docker;

fn main() {
    let mut docker = match Docker::connect() {
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
extern crate docker_sync;

use docker_sync::Docker;

fn main() {
    let mut docker = match Docker::connect() {
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
extern crate docker_sync;

use docker_sync::Docker;

fn main() {
    let mut docker = match Docker::connect() {
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
extern crate docker_sync;

use docker_sync::Docker;

fn main() {
    let mut docker = match Docker::connect() {
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
extern crate docker_sync;

use docker_sync::Docker;

fn main() {
    let mut docker = match Docker::connect() {
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
extern crate docker_sync;

use docker_sync::Docker;

fn main() {
    let mut docker = match Docker::connect() {
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
extern crate docker_sync;

use docker_sync::Docker;

fn main() {
    let mut docker = match Docker::connect() {
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

### Ping the docker server

```rust
extern crate docker_sync;

use docker_sync::Docker;

fn main() {
    let mut docker = match Docker::connect() {
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
extern crate docker_sync;

use docker_sync::Docker;

fn main() {
    let mut docker = match Docker::connect() {
    	Ok(docker) => docker,
        Err(e) => { panic!("{}", e); }
    };

    let version = match docker.get_version() {
        Ok(version) => version,
        Err(e) => {panic!("{}",e)}
    };
}
```

## Contributing

To have a consistent dev environment one can use the docker image in /devenv like so:
1. `git clone https://github.com/bpetit/rs-docker-sync`
2. `cd rust-docker/devenv`
3. `./build_docker` (this assumes your user can run docker commands, otherwise `sudo`)
4. `./run_docker -ti`
5. Already inside the container:
  1. `cd Code`
  2. `cargo test`
