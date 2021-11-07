Unofficial command line tool for using [catbox.moe](https://catbox.moe)'s API

# Usage

Use `catbox --help` to see usage.

Catbox has the following commands:
- upload: Upload to Catbox.
- delete: Delete files
- album: Album commands
- litter: Upload a temporary file to Litterbox.


The `album` subcommand has additional subcommands:
- create: Create a new album
- delete: Delete an album
- edit: Edit an album
- add: Add files to an album
- remove: Remove files from an album

You can use `--help` on any command to see its usage.

The basic `upload` command will accept both local files and URLs to files hosted somewhere else.

Some commands require an account hash to work. This can be supplied using
the `--user` argument or by setting `CATBOX_USER_HASH` environment value.
The explicitly provided argument will be preferred over the environment variable.
If the environment variable is set, it will be used even when optional.

All commands print the response from the server, usually a link to the created file or album.

See <https://catbox.moe/tools.php> for more information about the API and
<https://catbox.moe/faq.php> for allowed filetypes and content.

Consider donating via <https://www.patreon.com/catbox> to help with server costs.

# Examples

Upload a file:
```
catbox upload cute_picture.png
```

Delete a file:
```
catbox delete <file>.jpg --user 1234567890123456789012345
```

Create an album:
```
catbox album create --title 'My album' --desc 'An excellent album' abc123.jpg def456.png
```

The user argument is not needed if `CATBOX_USER_HASH` is found in environment.

Upload a file to Litterbox for 3 days:
```
catbox litter --time 72h homework.zip
```
