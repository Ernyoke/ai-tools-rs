# example-cmd

Give an example of a Linux (Unix) command based on a description.

## Usage

```bash
$ example-cmd -h
Usage: example-cmd [OPTIONS] <CMD_DESCRIPTION>

Arguments:
  <CMD_DESCRIPTION>  Short description of the command for which we would like an example. Example: list files which start with my_pic"

Options:
  -n, --nr-examples <NR_EXAMPLES>  Number of examples to be requested" [default: 1]
  -h, --help                       Print help
  -V, --version                    Print version
```

### Examples:

- Request a single example for a command:

```bash
$ example-cmd "run an nginx docker image in iterative mode with environment variables"
docker run -it --env VAR1=value1 --env VAR2=value2 nginx
```

- Request multiple examples:

```bash
$ example-cmd "list all processes" -n 3
1. ps aux
2. top
3. htop
```
