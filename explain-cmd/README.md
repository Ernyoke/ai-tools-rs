# example-cmd

Explain like I'm 5 a Linux (Unix) command.

## Usage

```bash
$ explain-cmd -h
Usage: explain-cmd <CMD>

Arguments:
  <CMD>  Linux (Unix) command. For example: ls -lart

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Examples:

```bash
$ explain-cmd "ls -lart"
This command will list the contents of the current directory in reverse order by modification time. The options used are:
-l (long listing format which includes file permissions, size, owner, group, modification time, and filename),
-a (show all files, including hidden files),
-r (reverse the order of the output), and
-t (sort by modification time).
```

## API KEY

Since this tool relies on OpenAI API, it requires an API key. The API key can be placed in a config file on the following 
path:

```
~/.config/openai/config.json
```

The structure of the `config.json` file should be the following:

```json
{
    "api-key": "sk-..."
}
```
