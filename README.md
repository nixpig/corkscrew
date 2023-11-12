# 🌀 corkscrew

Declaratively build and execute http requests.

> This is my first ever project in Rust. No doubt, doing some things poorly. Happy to receive constructive suggestions in [issues](https://github.com/nixpig/corkscrew/issues).

## Example

```shell
corkscrew -f requests.yml
  # => get request to https://jsonplaceholder.typicode.com/posts/1
  # => get request to https://jsonplaceholder.typicode.com/comments?postId=1
  # => post request to https://jsonplaceholder.typicode.com/posts with body { title, body, userId }
```

```yaml
# requests.yml
- host: jsonplaceholder.typicode.com
  scheme: https
  requests:
    - name: get_posts
      path: /posts/1

    - name: get_comments
      path: /comments
      params:
        postId: 1

    - name: create_post
      path: /posts
      method: post
      type: json
      body:
        title: foo
        body: Lorem ipsum dolar sit amet.
        userId: 1
```
