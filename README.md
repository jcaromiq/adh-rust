# ADH: Docker 'Useful Commands' Helper


##  Description

> This package will help you working with Docker.

> Port in Rust of original ADH from [Apiumhub](https://github.com/ApiumhubOpenSource/adh)
  
    

##  Installation

```
brew tap github/jcaromiq https://github.com/jcaromiq/taps
brew install adh
```
   
  
## Usage

```
$ adh --help

  Commands:

    nginx                     			Run nginx with a volume in the current directory 
    start <container_id>            Start container with given id
    stop <container_id>             Stop container with given id
    ps                         			Formatted ps for running dockers
    psa                        			Formatted ps for all dockers
    rc       			                  Remove all containers
    remove-none-images         			Remove none images

  Options:

    -h, --help     Prints help information
    -V, --version  Prints version information
```
