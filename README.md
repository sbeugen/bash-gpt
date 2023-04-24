# Bash GPT

This is a small program for finding a Bash command by explaining what you want to do in natural language.
It will send the provided query to Open AI's ChatGPT and print the result.

## Build

```
$ cargo build -r
```

This will produce the binary `bash-gpt` in the directory `/target/release`.

## API-Key

You need to provide an Open AI API key via the env variable `OPENAI_API_KEY`
You can also use a dotenv (`.env`) file.

```bash
$ cp example.env .env # then edit the file
```

## Run with arguments

- `--verbose` or `-v` - verbose mode will print explanations for the provided command

## Examples

1. Command: `bash-gpt find all files where the name contains foo`  
Output: `find . -name "*foo*" -type f`
2. Command: `bash-gpt -v reset git branch to commit`  
Output: 
```
The bash command for resetting a git branch to a specific commit is:

``
git reset --hard <commit hash>
``

This command will completely remove all changes and commits made after the specified commit and reset the branch to that specific commit. The `--hard` option means that any changes made after the specified commit will be permanently deleted and cannot be recovered. The `<commit hash>` is the unique identifier for the commit you want to reset to.

```
