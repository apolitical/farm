Cuckoo
======

A find and replace tool for MySql. Given a `--find` string, it will find all occurrences within a
a schema and replace it with a `--replace` string.

Usage
-----

```bash
$ cuckoo \
    --database mysql://<user>:<password>@<database>:3306/wordpress \
    --find https://apolitical.co \
    --replace http://localhost:8080
```
