# ADH: Docker 'Useful Commands' Helper
<p align="center">
  <a href="https://github.com/jcaromiq/adh-rust/actions/"><img src="https://github.com/jcaromiq/adh-rust/actions/workflows/ci.yml/badge.svg" alt="Build Status"></a>
  <a href="https://deps.rs/repo/github/jcaromiq/adh-rust"><img src="https://deps.rs/repo/github/jcaromiq/adh-rust/status.svg" alt="Dependency status"></a>
</p>

##  Description

This package will help you working with Docker.

Port in Rust of original ADH from [Apiumhub](https://github.com/ApiumhubOpenSource/adh)
  
    

##  Installation

```
brew tap jcaromiq/taps
brew install adh
```
   
  
## Usage

```
$ adh --help

  Commands:

    nginx                       Run nginx with a volume in the current directory
    mysql                       Run mysql
        --database_name         Optional. Allows you to specify the name of a database to be created on image startup
        --root_password         Optional. Specifies the password that will be set for the MySQL root superuser account
                                If not set, a random password will be created and printed at the end
    start <container_id>        Start container with id container_id, if container_id is not provider, user can select from a list
    stop <container_id>         Stop container with id container_id, if container_id is not provider, user can select from a list
    ps                          Formatted ps for running dockers
    psa                         Formatted ps for all dockers
    rc                          Remove all containers
    remove-none-images          Remove none images
    registry                    Create a local registry
    elastic                     Run an ElasticSearch
    ri                          Remove all named volumes
    rec                         Remove exited containers
    kc                          Kill all containers
    remove-volumes              Remove all volumes
    log <container_id>          Show container logs with id container_id, if container_id is not provider, user can select from a list
    flog <container_id>         Show container logs in follow mode with id container_id, if container_id is not provider, user can select from a list

  Options:

    -h, --help     Prints help information
    -V, --version  Prints version information
```
