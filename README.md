Farm
====

[![GitHub release](https://img.shields.io/github/release/apolitical/farm.svg)](https://github.com/apolitical/farm/releases)
[![GitHub license](https://img.shields.io/github/license/apolitical/farm.svg)](https://github.com/apolitical/farm/blob/master/LICENSE)

A find and replace tool for MySql. Given a `--find` string, it will find all occurrences within a
a schema and replace it with a `--replace` string.

Example Usage
-------------

```bash
$ farm \
    --database mysql://user:password@localhost:3306/wordpress \
    --find https://apolitical.co \
    --replace http://localhost:8080
```

Installation
------------

Clone this repository and run `cargo install`.

Docker
------

This repository is available on Docker

Usage:

```
docker run --rm -it apolitical/farm --database <db-url> --find <find> --replace <replace>
```

or

```
docker run --rm  apolitical/farm --database <db-url> --find <find> --replace <replace> -y
```

If your database is also in docker, don't forget `--link <my-db-container-name>`

The entry point is farm, so you do not need to add the binary name.

Environment variables
---------------------

Sometimes you may not be able to pass in variable command line arguments (for example, when using
farm in a [Kubernetes Job]). You can pass the above options in as the environmental variables
`DATABASE`, `FIND`, `REPLACE`, `RECKLESS_MODE`.

> **Warning:** The environment variables will be used for anything that isn't passed in on the
> command line. This is mostly fine, but `RECKLESS_MODE` doesn't take a parameter and therefore
> can not be turned off if you have an environment variable _called_ `RECKLESS_MODE` for any reason.

[Kubernetes Job]: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/
