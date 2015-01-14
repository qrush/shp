# shp

![](http://pirates.missiledine.com/images/ships/shipsection.jpg)

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
* blob
* staging area
* index
* remote

## Essential concepts

1. Everything is undoable
2. Checking in work is akin to loading it on a truck for delivery
3. Be centralized (use git if you want to be distributed)
4. No work should ever be lost
5. There is no need to be upset

## Commands

Trying to get this out of my head and on...paper?

| Command | Explanation |
| ------- | ----------- |
| `shp help <command>` | Gives help! |
| `shp --help <command>` | Also gives help as we are too used to Unixy commands |
| `shp setup` | Asks for and stores user name and email |
| `shp start [URL, directory, default = .]` | Starts a repo off |
| `shp undo` | Undo the last constructive/destructive command. Can also undo an undo. |
| `shp load [file, directory, default = .]` | Loads the pallet with a file to be checked in |
| `shp pack [message, open $EDITOR]` | Pack up this pallet for shipping. |
| `shp pallet` | See what's on your pallet. |
| `shp ship` | Send your unshipped pallets off. Ship shipping ship ships! |
| `shp unshipped` | See what pallets haven't been shipped yet |
| `shp log` | See a log of pallets |
| `shp port [address]` | Add a new port to ship to |

# Undo

Every command that changes things must have an undo.

| Last Command Ran | Explanation |
| ---------------- | ----------- |
| `shp start` | Offers to remove the repo |
| `shp undo` | Undo the last undo |
| `shp load` | Removes that file from the pallet |
| `shp pack` | Unwrap the pallet and stop it from being shipped |
| `shp ship` | Reverts to the last shipped pallet |