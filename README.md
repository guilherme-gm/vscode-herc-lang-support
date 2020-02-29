# Hercules Script Support

Adds support to Hercules' (https://herc.ws) scripting language.

## Main Issues
- Indenting with tabs is not supported because VSCode sends a column based on the tab-size, while tree-sitter gives us a position when tab-size is 8 characters.
> We may be able to work around that by sending a custom config with the tabsize (check client/extension.ts for extra info). And dealing with tabs manually in the server.
> Example: User triggers auto complete --> reads line char by char and when a tab is found, replace it by +8 (and reduce the number of columns from the range)

- Additional code in the same line as the header is not supported (because of the issue above).

- Actually all the above may be quite wrong, because further tests has shown that it actually measures tabs as 1 character... needs further testing and implementation

## Todos
- Add diagnostic for functions that end in ',' as the grammar allows it because of the signatur ehelper. e.g.:
```
mes("Hello",); //< the comma is invalid
```

## Features

### Syntax Highlighting

![Syntax Highlighting](images/highlight.gif)

### Snippets/Macros

![Snippets](images/snippets.gif)

The extension comes bundled with the following snippets to speed up repetitive tasks:

|       Name         |                                     Description                                  |
|--------------------|----------------------------------------------------------------------------------|
| npc-script         | Generates a script NPC that will be in a map                                     |
| npc-script-float   | Generates a floating script NPC                                                  |
| npc-script-trigger | Generates a script NPC in a map and with Trigger area                            |
| npc-dummy-script   | Generates a script NPC with all information filled, to be used on initial setups |
| npc-warp           | Generates a warp NPC                                                             |
| func               | Generates a function header                                                      |

### Code Completion / Documentation

![Code Completion and Documentation](images/complete_signature.gif)

It gives you a list of all Hercules' commands, along with their documentation
so you can quickly write your code. Also, the current parameter is highlighted
and extra information may be available.

![Variables Completion](images/variables.gif)

Completion is also provided for variables assigned in previous lines of the same NPC!

### Error Checking

![Error Checking](images/error.gif)

Invokes map-server to execute an error checking on your script file and reports it
back to you, pointing the error line and a message explaining the problem.

**Note:** This feature requires you to setup `mapserverFolderPath` (Check [Extension Settings](#extension-settings)).

## Extension Settings

This extension contributes the following settings:

* `hercscript.mapserverFolderPath`: Allows you to set the path to an Hercules' compiled
  map-server to be used by error checking. When defined it will run this map-server everytime
  the script is saved in order to provide error information.

## Known Issues

This extension is still in beta, and it is expected for issues to show up frequently. If you run
into an issue, check our Issues page and search if someone else have already reported it, if not,
feel free to open a new issue describbing what is happening.

## Feature Requests

Have an idea of a new feature? Open an issue describbing it at length, I'll be glad to read it
and if possible it may be implemented in the future.

## Is it on VSCode Marketplace?

No. Not yet. I don't think the current state of it is stable enough for it to be published
in VSCode Marketplace.

## Contributting

Want to help making the extension better? Your help is really appreciated!

There are a couple ways to contribute to this extension, even if you don't know how to code!

### Improve commands documentation
Hercules' script engine has LOTS of commands, and we NEED TO document all of them, list their parameters
returns, how to use, etc etc. Currently we have a automatically generated documentation for most commands,
although those documentations kind of works, they are not 100% accurate and some times they miss important
information.

One way to help the extension is by taking your time to pick a command and rewrite its documentation
correctly and with all the details required. Check @@TODO issue to see the list of commands that are
still missing a documentation.

To contribute to the docs, you must check the Command Documentation Standard (@@TODO: Link)

### Implementing new features
Language Server Protcol has lots of functionalities, and we could implment them all! But doing this takes
a lot of time. So you may want to take the task of implementing one, or a part of it! I just ask that
before actually doing it, please open an issue about it and lets discuss!

Discussing it we may find a good way to make it work, and also you get an assigment to that task so there's
no risk of someone else (or even me!) working in the same feature and someone doing a lot of work for nothing!

## Release Notes

Check [CHANGELOG.md](CHANGELOG.md) files for release notes.
