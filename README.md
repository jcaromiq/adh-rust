# ADH: Docker 'Useful Commands' Helper


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
    start <container_id>        Start container with given id
    stop <container_id>         Stop container with given id
    ps                          Formatted ps for running dockers
    psa                         Formatted ps for all dockers
    rc                          Remove all containers
    remove-none-images          Remove none images
    clr                         Create a local registry
    ri                          Remove all named volumes
    rec                         Remove exited containers
    kc                          Kill all containers

  Options:

    -h, --help     Prints help information
    -V, --version  Prints version information
```
