# shp

![](https://user-images.githubusercontent.com/12610/127021727-166ff0f3-5959-4771-8cb3-48917e47d50b.jpg)

A new UI for git. 

## Goals

1. To provide a more human user interface for git
2. Interoperate with existing git tooling
3. Enable the community to use modern ways to contribute

## UI Ideas

* Break the `git` command up
* Stop the command line flag overload
* Provide undo for *all* destructive commands
* Smart defaults and aliases out of the box
* Banish the use of SHAs (not human!)
* Keep the index but use the loading dock metaphor

## Implementation Ideas

* Use Rust and libgit2 bindings for CLI
* Test suite in Ruby - because why not
* Provide binaries for Win/OSX/Linux
* Figure out what the best docs are
* Use revision numbers like `hg` does

## Banned git jargon

All of this is "under the hood" stuff. We don't need to worry about it.

* SHA
* commit
* tree(ish)
* "ref"
* blob
* staging area
* index
* remote
* repo

## Essential concepts

1. Everything is undoable
2. Checking in work is akin to loading it on a truck for delivery
3. Be centralized (use git if you want to be distributed)
4. No work should ever be lost
5. Be consistent in how data is added, removed, and renamed
6. There is no need to be upset

## Commands

Trying to get this out of my head and on...paper?

### General commands

| Command | Explanation |
| ------- | ----------- |
| `shp help <command>` | Gives help! |
| `shp --help <command>` | Also gives help as we are too used to Unixy commands |
| `shp setup` | Asks for and stores user name and email |
| `shp undo` | Undo the last constructive/destructive command. Can also undo an undo. |

### Write commands

| Command | Explanation |
| ------- | ----------- |
| `shp start [URL, directory, default = .]` | Starts a port off |
| `shp [un]load [file, directory, default = .]` | Loads/unload the pallet with a file/directory |
| `shp pack [message, open $EDITOR]` | Pack up this pallet for shipping. |
| `shp repack` | Modify your last pack |
| `shp dock [add, remove, rename, switch] [name]` | Add/remove/rename current dock |
| `shp port [add, remove, rename, switch] [name] [address]` | Add/remove/rename a port to ship to |
| `shp send [port] [dock, mark]` | Send your unshipped pallets off. Ship shipping ship ships! |
| `shp rec[eive] [port] [dock]` | Haul pallets in from a port (or the default port) |
| `shp mark [add, remove, rename] [name] [message]` | Marks the pallet with a message or for release. |

### Read commands

| Command | Explanation |
| ------- | ----------- |
| `shp pallet` | See what's on your pallet. |
| `shp dock` | Show current dock (branch) |
| `shp what [address1] [address2?]` | What is different between the two addresses or your current pallet? |
| `shp log [type]` | See a log of pallets. Log types: graph, patches, lines |
| `shp summary` | Get a mini log of everything in this port |
| `shp view [address]` | See the changes from the pallet at address. |
| `shp ports` | List ports we can ship to |

# What's missing

* How to query ports for docks, remove them
* Merge/rebasing, we need to figure that out.
* Server, daemon, etc. Use git for now.
* Patches, patch workflow.
* Ignoring?

# Undo

Every command that changes things must have an undo.

| Last Command Ran | Explanation |
| ---------------- | ----------- |
| `shp start` | Offers to remove the port |
| `shp undo` | Undo the last undo |
| `shp load` | Removes that file from the pallet |
| `shp unload` | Puts that file back on the pallet |
| `shp pack` | Unwrap the pallet and stop it from being shipped |
| `shp send` | Reverts to the last shipped pallet |
| `shp repack` | Roll back last changed pallet |
