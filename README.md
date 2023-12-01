# 🌀 corkscrew

A simple tool for executing HTTP requests configured in YAML. Written in Rust, btw.

> 🦀 This is my first ever project in Rust. No doubt there's going to me many things that can be improved upon. Happy to receive constructive suggestions in [issues](https://github.com/nixpig/corkscrew/issues).

## Examples

**Execute all requests in `requests.yml` file:**

```shell
$ corkscrew --help

Configure HTTP requests in YAML and execute from the command line.

Usage: corkscrew [OPTIONS] [REQUEST_NAMES]...

Arguments:
  [REQUEST_NAMES]...

Options:
  -f, --file <file_path>        Path to file containing requests [default: requests.yml]
  -p, --parallel <num_threads>  Specify number of parallel threads
  -h, --help                    Print help
  -V, --version                 Print version
```

```shell
$ corkscrew
  # => get request to https://example.com/posts/1
  # => get request to https://example.com/comments?postId=1
  # => post request to https://example.com/posts with body { title, body, userId }
```

**Execute specific named request(s) in `foo.yml` file:**

```shell
$ corkscrew -f foo.yml get_comments create_post
  # => get request to https://example.com/comments?postId=1
  # => post request to https://example.com/posts with body { title, body, userId }
```

```yaml
# requests.yml
- name: get_posts
  host: example.com
  resource: /posts/1

- name: get_comments
  host: example.com
  resource: /comments
  params:
    postId: 1

- name: create_post
  host: example.com
  resource: /posts
  method: post
  auth: !bearer
    token: abcd$1234.231&4dfs-asdfjsdv.vsd
  body:
    title: Lorem ipsum
    body: Lorem ipsum dolar sit amet.
    userId: 1

- name: update_post_title
  host: example.com
  resource: /posts/1
  method: patch
  auth: !basic
    username: corks
    password: p4ssw0rd
  body:
    title: Dolar sit
```

Requests can also be nested, where descendents can 'inherit' and/or 'override' properties from their ancestors.

```yaml
- name: example_root
  host: example.com
  resource: /api
  requests:
    - name: example_get_post
      resource: /api/post
    - name: example_get_comments
      resource: /api/comments
      params:
        postId: 1
```

Which would result in:

```shell
$ corkscrew
  # => get request to https://example.com/api
  # => get request to https://example.com/api/post
  # => get request to https://example.com/api/comments
```

## Installation

1. `git clone https://github.com/nixpig/corkscrew.git`
1. `cd corkscrew`
1. `cargo build --release`
1. `mv target/release/corkscrew ~/.local/bin/`

```shell

```

## API

Currently implemented. This is a work in progress and open to change.

```yaml
- name: Required<string> # name of the host (can be any string, it's not used to build the actual request)
  host: Optional<string> # the host to which to make a request, e.g. example.com
  scheme: Optional<http|https> # the scheme to use, e.g. https (default: http)
  port: Optional<number> # the port to use
  timeout: Optional<number> # number of seconds before timing out (default: 30)
  resource: Required<string> # that resource to request, e.g. /api/user
  method: Optional<get|post|put|patch|delete> # the http method to use, e.g. !post (default: !get)
  params:
    # <parameter_name>: <parameter_value>
    name: value

  # the type of authentication to use, valid values are !basic or !bearer
  auth: !auth_type # valid enum values are !basic or !bearer
    token: Optional<string> # in the case of !bearer authentication, provide the token to use
    username: Optional<string> # in the case of !basic authentication, provide the username to use
    password: Optional<string> # in the case of !basic authentication, provide the password to use

  # Optional body content (parsed to JSON)
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

- Add option to parallelise request execution and specify number of threads, e.g. `--parallel 4`.
- Add option to output various data from request response.
- Show some 'in progress' message/counter.
- Improve structure and algorithm of request struct recursion.
- Add option to specify nested groups of requests by `request_group` / `request_group:request_group` / `request_group:request`.
- Make the parser option configurable, e.g. `corkscrew --parser json`, to implement different parsers instead of YAML.
