- I want Terse to have a multimodal approach; you should be able to run commands quickly; ```ts --pub``` and so on, or type in ```ts``` to open up a full TUI with different tabs for a more intuitive interface. Plus, this might behoove good design and modularity in the system, something to look forward to.
    - I might be able to have a 'get string' fn for, say, stats, and then optionally feed it into a Line to render in the TUI, OR have the getString function return a Line and call render() just once to render it on-terminal.

- Module structure module structure module structure
    - Having, say, all this Post stuff in a single file is unideal; but what, a pub file, a Post file? all in a mod? What names do I give these files is the question? Two hard problems, amirite?

- Docs README claims that these are effectively the man pages. How do you integrate them to be actual man pages?...
    - I want Terse's help to be actually human-readable...

- This is an idea: 
    https://docs.rs/clap-help/latest/clap_help/

- mpsc seems the best bet. Maybe spsc or something to isolate and send messages.