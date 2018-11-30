# check_redis
[![Build Status](https://travis-ci.com/lagooj/check_redis.svg?branch=master)](https://travis-ci.com/lagooj/check_redis)

## Purpose

Just trying to write some rust.

## Usage

Check redis status and memory usage, should be runned by icinga, nagios

USAGE:
    check_redis [FLAGS] --critical <CRITICAL> --hostname <HOSTNAME:PORT> --warning <WARNING>

FLAGS:
    -h, --help       Prints help information
    -v               Set verbose mode
    -V, --version    Prints version information

OPTIONS:
    -c, --critical <CRITICAL>         Set critical threshold
    -H, --hostname <HOSTNAME:PORT>    Set hostname and port ex: localhost:6379
    -w, --warning <WARNING>           Set warning threshold

