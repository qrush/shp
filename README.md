# lockjaw

![](https://tarangini.files.wordpress.com/2011/05/headmuscles1.jpg)

A new UI for git. With less pain.

(Also to prove we're all locked into git now for at least another few years. And Rust is neat.)

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

## Unallowed git jargon

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
| `jaw help <command>` | Gives help! |
| `jaw --help <command>` | Also gives help as we are too used to Unixy commands |
| `jaw setup` | Asks for and stores user name and email |
| `jaw start [URL, directory, default = .]` | Starts a repo off |
| `jaw undo` | Undo the last constructive/destructive command. Can also undo an undo. |
| `jaw load [file, directory, default = .]` | Loads the pallet with a file to be checked in |
| `jaw pack [message, open $EDITOR]` | Pack up this pallet for shipping. |
| `jaw pallet` | See what's on your pallet. |
| `jaw ship` | Send your unshipped pallets off |
| `jaw unshipped` | See what pallets haven't been shipped yet |

# Undo

Every command that changes things must have an undo.

| Last Command Ran | Explanation |
| ---------------- | ----------- |
| `jaw start` | Offers to remove the repo |
| `jaw undo` | Undo the last undo |
| `jaw load` | Removes that file from the pallet |
| `jaw pack` | Unwrap the pallet and stop it from being shipped |
| `jaw ship` | Reverts to the last shipped pallet |