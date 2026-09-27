# The inner workings of Terse

The main entry point is of course src/main.rs. This does nothing much but reimport some things and create and run an instance of the Cli struct.

The Cli struct is basically a clap parser that you create and call process() on to run the whole shebang. Small operations are commands in Cli, but for subcommands, those are split into individual files in src/cli/ that Cli calls

Before and after processing, Cli gets a ServerList from a data file (in /data in debug builds, in the proper XDG path in release builds). This runs some extra stuff internally if the file doesn't exist, et cetera et cetera. Cli stuffs this ServerList into an Arc<parking_lot::RwLock< >> because references to the ServerList are needed in a lot of the program and I don't want to have to deal with lifetimes.

## The ServerList

This is gathered from a file at the start of the Cli's process() function (terminating if it can't figure out how to get it) and fed throughout the parts of the program that need it, enclosed in an Arc<parking_lot::RwLock< >>. It stores a blocking reqwest Client and a list of Servers. Servers are just a url and additional account LoginInfo. They have no internal code and are hopefully cheap to copy around, like little tags. The ServerList also stores a 'selected' index into the 'current' server which you can query.

At the end of process(), the ServerList (possibly modified by the program) is stuffed back into the file it came from

## The Tui

All the tui is stored in /tui, oddly enough. Whenever a tui is used in the program, an instance of the App struct is made and run() is called on it, passing in a struct that has the Component trait.
The App is responsible for catching input events and telling the Component what to do. It limits the Component's render area a bit to supply the escape-to-quit bottom bar.

### The Component trait

Ratatui uses an immediate rendering system. I don't like that, so Component is your go-to class for anything that:

- needs to render to the screen
- needs to respond to key events; I.E. has an internal state

pretty much. 

It can store subcomponents and tell those to render when it's told to render, very modular. It also returns a Action type (custom to each impl of Component) that the parent Component can use to do things. Plus it has some workings with Labels and all so the parent can call pre-defined functions in it that wrap render() to overlay help messages / key shortcuts.
This is the main reason for the Arc<RwLock<ServerList>> stuff; Components often need a reference to the ServerList, so they gotta store it without having to deal with all those mean old lifetimes getting in the way. This makes the developer experience much better.


