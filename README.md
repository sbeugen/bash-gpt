# Bash GPT - A Natural Language Interface for Bash Commands

This is a small program for finding a Bash command by explaining what you want to do in natural language.
It will send the provided query to OpenAI's ChatGPT and print the result.
The command itself will not be executed.

## Getting Started

To build the program, run the following command:

```
$ cargo build -r
```

This will produce the binary `bash-gpt` in the directory `/target/release`.

You will also **need** to provide an OpenAI API Key via the env variable `OPENAI_API_KEY`. You can either set this variable in your environment or use an dotenv (`.env`) file. To use a dotenv file, run the following command:

```bash
$ cp example.env .env # then edit the file
```

_Optionally_, you may want to create a symlink for the binary `bash-gpt` to some directory in your path, so that you can run the program from anywhere in your terminal. Here's an example command for creating a symlink: `ln -s /path/to/this/repo/sbeugen/bash-gpt/target/release/bash-gpt /usr/local/bin`

## Examples

Here are some examples of how to use the program:

```bash
$ # Command: find all files where the name contains foo
$ bash-gpt find all files where the name contains foo
find . -name "*foo*" -type f

$ # Command: reset git branch to commit
$ bash-gpt -v reset git branch to commit
The bash command for resetting a git branch to a specific commit is:

``
git reset --hard <commit hash>
``

This command will completely remove all changes and commits made after the specified commit and reset the branch to that specific commit. The `--hard` option means that any changes made after the specified commit will be permanently deleted and cannot be recovered. The `<commit hash>` is the unique identifier for the commit you want to reset to.
```

## Limitations

The program can handle a wide range of queries, but it may struggle with more complex or ambiguous requests. Additionally, the program is limited by the accuracy and completeness of OpenAI's ChatGPT model.