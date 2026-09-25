# Network notes

Since you probably don't want people to be able to look over your shoulder at your passwords, I use a flag (-p) to show them optionally. This means I need to somehow have the things display either with or without the password(s), which the Display trait doesn't support; traits have fixed argument schemes.

So I impl a custom as_string(show_password(s): bool) function on LoginInfo, Server, and ServerList that write!s to a String to get the desired result. As per the rules of D.R.Y. I manage to get Display to just call as_string(false), which is pretty nice.

the one for Server also contains a selected: bool argument for if it is the current server, which gives it come extra flair.

I always .expect() the writes to the Strings; it shouldn't ever have an input stream error and [this post](https://users.rust-lang.org/t/write-to-a-string-failure-documentation/78293) seems to give more weight to that theory