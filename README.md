Cuckoo
======

A find and replace tool for MySql. Given a `--find` string, it will find all occurrences within a
a schema and replace it with a `--replace` string.

Example Usage
-------------

```bash
$ cuckoo \
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
docker run --rm -it apolitical/cuckoo --database <db-url> --find <find> --replace <replace>
```

or

```
docker run --rm  apolitical/cuckoo --database <db-url> --find <find> --replace <replace> -y
```

If your database is also in docker, don't forget `--link <my-db-container-name>`

The entry point is cuckoo, so you do not need to add the binary name.
