```shell
# Run all all requests for default host in corkscrew.yml
corkscrew

# Run specific requests for default host
corkscrew login update_user_email

# Specify a host
corkscrew -h default
# Specify a host and requests to run
corkscrew -h default login update_user_email

# Specify a requests file
corkscrew -f requests.yml
corkscrew -f requests.yml create_post

# Pass in vars
corkscrew -e FOO=BAR # Access in yaml as {{ env.FOO }}

```
