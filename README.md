# 🌀 corkscrew

Declaratively build and execute http requests.

> This is my first ever project in Rust. No doubt, doing some things poorly. Happy to receive constructive suggestions in [issues](https://github.com/nixpig/corkscrew/issues).

## Examples

**Execute all requests in `hosts.yml` file:**

```shell
corkscrew
  # => get request to https://jsonplaceholder.typicode.com/posts/1
  # => get request to https://jsonplaceholder.typicode.com/comments?postId=1
  # => post request to https://jsonplaceholder.typicode.com/posts with body { title, body, userId }
```

**Execute specific requests in `requests.yml` file:**

```shell
corkscrew -f requests.yml get_comments
  # => get request to https://jsonplaceholder.typicode.com/comments?postId=1
```

```yaml
# hosts.yml
- name: example
  host: example.com
  requests:
    - name: get_posts
      resource: /posts/1

    - name: get_comments
      resource: /comments
      params:
        postId: 1

    - name: create_post
      resource: /posts
      method: !post
      auth: !bearer
        token: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjE3MDAyMDU4MjIsImlzX2FkbWluIjpmYWxzZSwidXNlcl9pZCI6Mn0.1uKA2rzEoOajZ5bBnxes9XIUo2iOwKCD7bVVwvRzJ48
      body:
        title: Lorem ipsum
        body: Lorem ipsum dolar sit amet.
        userId: 1

    - name: update_post_title
      resource: /posts/1
      method: !patch
      auth: !basic
        username: corks
        password: p4ssw0rd
      body:
        title: Dolar sit
```

## Installation

## API

Currently implemented. This is a work in progress and open to change.

```yaml
- name: # name of the host (can be any string, it's not used to build the actual request)
  host: # the host to which to make a request, e.g. example.com
  scheme: Optional<http|https> # the scheme to use, e.g. https (default: http)
  port: Optional<number> # the port to use
  timeout: Optional<number> # number of seconds before timing out (default: 30)
  requests:
    - name: Required<string> # the name of the request, e.g. get_user_by_id
      resource: Required<string> # that resource to request, e.g. /api/user
      method: Optional<Enum<!get|!post|!put|!patch|!delete> # the http method to use, e.g. !post (default: !get)

      # Optional request parameters
      params:
        # <parameter_name>: <parameter_value>
        name: value

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

Yes, I've also tried _{insert favourite util here}_, raw-dogging `ncat` requests, and everything in between.

## Contribute

This is a personal project for me to learn Rust and is not currently open for contributions.

This may change in the future.

Feel free to leave constructive comments, feedback or suggestions in the [issues](https://github.com/nixpig/corkscrew/issues).
