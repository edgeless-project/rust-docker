# This is a fork
This repo is a fork of [https://gitlab.com/kblobr/rust-docker](https://gitlab.com/kblobr/rust-docker).

On the EDGELESS platform, to run trusted function images using [SecureExecutor](https://github.com/edgeless-project/SecureExecutor),
it is necessary to pass the SGX driver to the container during their initiation. 

This allows the containers to leverage Intel SGX within the generated images. 

However, the latest version of `rs-docker` (0.0.60) does not support `Devices` out of the box. 
Therefore, this repo contains the required updated to support this functionality.

Relevant [MVP](https://github.com/edgeless-project/edgeless.git) issue: [https://github.com/edgeless-project/edgeless/issues/226](https://github.com/edgeless-project/edgeless/issues/226)

Original `rs-docker` [README.md](./README-original.md)
