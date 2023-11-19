# 🌀 corkscrew

Declaratively build and execute http requests.

> This is my first ever project in Rust. No doubt, doing some things poorly. Happy to receive constructive suggestions in [issues](https://github.com/nixpig/corkscrew/issues).

## Examples

**Execute all requests in file:**

```shell
corkscrew -f requests.yml
  # => get request to https://jsonplaceholder.typicode.com/posts/1
  # => get request to https://jsonplaceholder.typicode.com/comments?postId=1
  # => post request to https://jsonplaceholder.typicode.com/posts with body { title, body, userId }
```

**Execute specific requests in file:**

```shell
corkscrew -f requests.yml get_comments
  # => get request to https://jsonplaceholder.typicode.com/comments?postId=1
```

```yaml
# requests.yml
- name: jsonplaceholder
  host: jsonplaceholder.typicode.com
  scheme: https
  port: 3001
  timeout: 10
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
        type: json
        content:
          title: foo
          body: Lorem ipsum dolar sit amet.
          userId: 1
```

## Installation

## API (proposed, but not yet implemented)

```yaml
- host: Required<string> # the host to which to make a request, e.g. jsonplaceholder.typicode.com
  scheme: Optional<http|https> # the scheme to use, e.g. https (default: http)
  port: Optional<number> # the port to use
  timeout: Optional<number> # number of seconds before timing out (default: 30)
  requests:
    - name: Required<string> # the name of the request, e.g. get_user_by_id
      resource: Required<string> # that resource to request, e.g. /api/user
      method: Optional<Enum<!get|!post|!put|!patch|!delete> # the http method to use, e.g. !post (default: !get)
      type: Optional<json> # shorthand to specify "Content-Type: application/json" (default: text)

      # Optional request parameters (parsed as name=value)
      params:
        name: value

      # Optional body content (parsed as JSON)
      body:
        name: value # <property_name>: <property_value>
        # also supports nested JSON structures
        l1_name:
          l2_name:
            l3_name1: value
            l3_name2:
              l4_name1: value
              l4_name2: value

      # Optional headers
      headers:
        # <header_name>: <header_value>
        name: value
```

## Motivation

## Alternatives

There are a bunch of other solutions for making REST API requests which are defined in config, just to name a few of the most common:

| Name    | Type |
| ------- | ---- |
| cURL    | CLI  |
| Postman | GUI  |
| HTTPie  | CLI  |

## Contribute

This is a personal project for me to learn Rust and is not currently open for contributions.

This may change in the future.

Feel free to leave constructive comments, feedback or suggestions in the [issues](https://github.com/nixpig/corkscrew/issues).
