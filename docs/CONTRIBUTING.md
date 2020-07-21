# Contributing

First off, thank you for considering contributing to poke-speare.

If your contribution is not straightforward, please first discuss the change you
wish to make by creating a new issue before making the change.

## Docker

In order to work on the docker image locally you can build and run the image
with the following steps:

```sh
docker build --tag=poke-speare .
docker run -p 5000:5000 poke-speare
```
