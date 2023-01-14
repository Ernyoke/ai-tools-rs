# ai-tools-rs

Collection of CLI helper tools using OpenAI API.

## API KEY

These tools require a valid OpenAI API key which can be requested here: https://beta.openai.com/account/api-keys. The API key should be placed in a config file on the following path:

```
~/.config/openai/config.json
```

The structure of the `config.json` file should be the following:

```json
{
    "api-key": "sk-..."
}
```

