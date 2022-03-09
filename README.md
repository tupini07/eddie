# Eddie

A command manage you can use from your terminal! Create a series of `toml` files defining the _groups_ you want, and each group
can either have nested groups inside of it or _commands_. 

_Eddie_ provides a nice interface to read and interact these toml files. It allows you to navigate the different groups and execute
the commands.

> NOTE: right now Eddie only works with a global set of toml file, but in the future it would be great to have project-specific files
> so you can have Eddie load different sets of toml files depending on what you're working on! This is being tracked in #17.

## Example

![image](https://user-images.githubusercontent.com/3422347/157512420-4d28b143-f618-4f37-b091-6c2372034f88.png)

## Installing

Right now you need to download, compile, and install the project using `cargo`. Luckily this is very easy, and can be done with:

```bash
git clone git@github.com:tupini07/eddie.git
cd eddie
cargo install --path .
```

Now you should have an `eddie` command globally available on your terminal 🤗

## Configuration

The configuration for Eddie is given as a set of toml files that live inside `~/.config/eddie/`. When eddie loads it will load all toml
files it finds here, concatenate them into one big file, and then parse it to construct the group/command tree. This means you can name
the files whatever you want, and can also use as many subfolders as desired. 

An example structure for config files can be something like:

```
~/.config/eddie/
            ├── main.toml
            ├── system.toml
            ├── tests.toml
            └── work
                ├── general.toml
                ├── project1.toml
                └── project2.toml
```

> see the configuration under `test_configuration` in the root of the repo to get an idea of how the config works
### `ship` table

Eddie only requires you to define **one** toml table to work properly, which should be named `ship`, and contains basic config information
for the application to work. This is an example:

```toml
[ship]
# the name of your "eddie" instance
name = "Goldar" 

# what application will be used if you tell eddie to execute a command in an external terminal
terminal_emulator = "C:/Users/username/scoop/shims/alacritty"
terminal_emulator_command_arg = "--command"

# what default shell should be used to run normal commands (must be on your path)
shell = "powershell"
```

### `group` tables

You can then define a set of tables to hold other tables or commands. The tables that act as containers have the following structure:

```toml
[some_table_key] # replace with whatever you want
# both 'name' and 'description' are used to populate Eddie's UI
name = "the name of the table" 
description = "description of what the table contains"
```

### `command` tables

Commands are represented with tables very similar to `group` tables but have some extra properties. Also, **you cannot nest a table inside a command table** (i.e. they must be _leafs_ of the configuration tree).

```toml
[parent_table_key_if_any.command_key] # replace with whatever you want
name = "name for the command"
description = "description of what the command does"
command = "actual command we want to execute"

# 'external' tells Eddie whether it can execute the command inside it's own process or a new terminal should be spawned.
# a good rule of thumb is that if the command is a long running process then set this to 'true'
# note: this defaults to 'false', so you don't need to specify it if you don't want to run in an external terminal
external = true 
```

> NOTE: you can also have commands on the top level of Eddie by not nesting them inside other tables.


## Keybindings

You can see the keybindings at the bottom of the terminal screen directly inside Eddie, but for convenience, here they are:

- `tab` to move to next item in list
- `shift+tab` to move to the previous item in list
- `enter` to select item 
  - This will dive into the group if the highlighted element is a group
  - Or it will execute the command if the highlighted element is a command
- `backspace` to make Eddie move one level up in the current group tree
  - If you're already at the top level then this will not do anything