# lockjaw

![tetanus-lockjaw-victim-the-contracted-everett](https://cloud.githubusercontent.com/assets/12610/5716125/5ee2ad54-9ab4-11e4-8f09-31aa64251b0d.jpg)

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

## Commands

Trying to get this out of my head and on...paper?

Command | Explanation
`jaw start` | Starts a repo off