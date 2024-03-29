[![Workflow Status](https://github.com/nixpig/corkscrew/actions/workflows/general.yml/badge.svg?branch=main)](https://github.com/nixpig/corkscrew/actions/workflows/general.yml?query=branch%3Amain)
[![Coverage Status](https://coveralls.io/repos/github/nixpig/corkscrew/badge.svg?branch=main)](https://coveralls.io/github/nixpig/corkscrew?branch=main)

# 🌀 corkscrew

A simple tool for executing HTTP requests configured in YAML. Written in Rust, btw.

> 🦀 This is my first ever project in Rust. No doubt there's going to me many things that can be improved upon. Happy to receive constructive suggestions in [issues](https://github.com/nixpig/corkscrew/issues).

## ⚠️ WORK IN PROGRESS

**It's probably not a good idea to actually use this for anything at this point. Maybe soon 🤷**

## Features

```shell
$ corkscrew --help

Configure HTTP requests in YAML and execute from the command line.

Usage: corkscrew [OPTIONS] [REQUEST_NAMES]...

Arguments:
  [REQUEST_NAMES]...

Options:
  -f, --file <file_path>  Path to file containing requests [default: requests.yml]
  -p, --parallel <num>    Specify number of parallel requests
  -h, --help              Print help
  -V, --version           Print version
```

## Installation

### Build from source

1. Install the Rust toolchain ([instructions](https://rustup.rs/))
1. `git clone https://github.com/nixpig/corkscrew.git`
1. `cd corkscrew`
1. `cargo build --release`
1. `mv target/release/corkscrew ~/.local/bin/`

```shell

```

## Examples

### Minimal example (single request)

```yaml
# requests.yml

- name: get_posts
  host: example.com
  resource: /api/posts
```

```shell
$ corkscrew
  # => get request to http://example.com/api/posts

+---+---------------------------------+--------+
| # | URL                             | Status |
+---+---------------------------------+--------+
| 0 | http://example.com/posts        | 200    |
+---+---------------------------------+--------+
```

### Multiple requests

```yaml
# requests.yml

- name: multiple_requests
  host: example.com
  requests:
    - name: get_users
      resource: /api/users
    - name: get_posts
      resource: /api/posts
    - name: get_comments
      resource: /api/comments
```

```shell
$ corkscrew
  # => get request to http://example.com/api/users
  # => get request to http://example.com/api/posts
  # => get request to http://example.com/api/comments
```

### Specify requests

```yaml
# requests.yml

- name: multiple_requests
  host: example.com
  requests:
    - name: get_users
      resource: /api/users
    - name: get_posts
      resource: /api/posts
    - name: get_comments
      resource: /api/comments
```

```shell
$ corkscrew get_posts get_comments
  # => get request to http://example.com/api/posts
  # => get request to http://example.com/api/comments
```

### Send POST request with JSON body

```yaml
# requests.yml

- name: post_json_body
  host: example.com
  resource: /api/likes
  method: post
  body:
    postId: 2
    userId: 3
```

```shell
$ corkscrew
  # => post request to http://example.com/api/likes
  # Content-Type: application/json
  # { "postId": 2, "userId": 3 }
```

### Send POST request with form data

```yml
# requests.yml

- name: post_form_data
  host: example.com
  resource: /api/comments
  method: post
  form:
    userId: 3
    comment: I really liked this!
```

```shell
$ corkscrew
  # => post request to http://example.com/api/comments
  # Content-Type: application/x-www-form-urlencoded
  # userId=3&comment=I%20really%20liked%20this%21
```

### Send request with query parameters

```yaml
# requests.yml

- name: query_params
  host: example.com
  resource: /api/comments
  params:
    userId: 3
    limit: 10
```

```shell
$ corkscrew
  # => get request to http://example.com/api/comments?userId=3&limit=10
```

### Send request with auth token

```yaml
# requests.yml

- name: bearer_auth
  host: example.com
  resource: /api/users
  auth: !bearer
    token: abcd$1234
```

```shell
$ corkscrew
  # => get request to http://example.com/api/users
  # Authorization: Bearer <token>
```

### Send request with HTTP basic auth

```yaml
- name: basic_auth
  host: example.com
  resource: /api/login
  auth: !basic
    username: corks
    password: p4ssw0rd
```

```shell
$ corkscrew
  # => get request to http://example.com/api/users
  # Authorization: Basic <credentials>
```

### Nesting requests

Requests can also be nested, where descendents can 'inherit' and/or 'override' properties from their ancestors.

```yaml
- name: example_root
  host: example.com
  resource: /api
  scheme: https
  requests:
    - name: example_get_post
      resource: /api/post
    - name: example_get_comments
      resource: /api/comments
      params:
        postId: 1
```

```shell
$ corkscrew
  # => get request to https://example.com/api
  # => get request to https://example.com/api/post
  # => get request to https://example.com/api/comments
```

## API

> This is a work in progress and open to change.

```yaml
- name: String # name of the host (can be any string, it's not used to build the actual request)
  host: String # the host to which to make a request, e.g. example.com
  scheme: String<http|https> # the scheme to use, e.g. https (default: http)
  port: Number # the port to use
  timeout: Number # number of seconds before timing out (default: 30)
  resource: String # that resource to request, e.g. /api/user
  method: String<get|post|put|patch|delete> # the http method to use, e.g. post (default: get)
  params:
    # <parameter_name>: <parameter_value>
    name: value

  # the type of authentication to use, valid values are !basic or !bearer
  auth: Enum<!basic|!bearer> # valid enum values are !basic or !bearer
    token: String # in the case of !bearer authentication, provide the token to use
    username: String # in the case of !basic authentication, provide the username to use
    password: String # in the case of !basic authentication, provide the password to use

  # Optional form data content (implicitly sets Content-Type: application/x-www-form-urlencoded)
  form:
    name1: value1
    name2: value2
    name3: value3

  # Optional body content (implicityly sets Content-Type: application/json)
  body:
    name: value # <property_name>: <property_value>
    # also supports nested JSON structures
    l1_name1: l1_value1
    l1_name2:
      l2_name1: l2_value1
      l2_name2:
        l3_name1: l3_value1
        l3_name2: l3_value2

  # Optional headers
  headers:
    # <header_name>: <header_value>
    name: value

  # Optional nested requests
  requests:
    - <Request>
```

## Motivation

I wanted a way to define project-specific REST API requests to quickly and easily execute from the command-line.

## Alternatives

There are a bunch of other solutions for making REST API requests.

I've tried most of these in the past, but they didn't quite fit into my workflow as I needed. They may well work for you, in which case, don't worry!

| Name    | Type | Challenges                                                                                              |
| ------- | ---- | ------------------------------------------------------------------------------------------------------- |
| cURL    | CLI  | Depending on the approach, either searching through shell history or maintaining lots of `.http` files. |
| Postman | GUI  | GUI apps don't fit well into my workflow. JSON is a pain to maintain.                                   |

Yes, I've probably also tried _insert favourite util here_, raw-dogging `ncat` requests, and everything in between.

## Contribute

This is a personal project for me to learn Rust and is not currently open for contributions.

This may change in the future.

Feel free to leave constructive comments, feedback or suggestions in the [issues](https://github.com/nixpig/corkscrew/issues).

## TODO

- [ ] Use Hyper instead of Reqwest.
- [ ] Handle errors and non-happy path scenarios.
- [ ] Use a request client pool to reuse clients instead of creating new ones for every request.
- [ ] Add option to parallelise request execution and specify number of threads, e.g. `--parallel 4`.
- [ ] Add option to output various data from request response.
- [ ] Show some 'in progress' message/counter.
- [ ] Add option to specify nested groups of requests by `request_group` / `request_group:request_group` / `request_group:request`.
- [ ] Get rid of all the data cloning, unwrapping, etc...
