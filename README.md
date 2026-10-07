<div align="center">

<h1> Terse </h1>
<sub>Terse is still in development, and nowhere near finished yet! Some of the features here don't even exist yet! <sub>
</div>

<sub>Note that this is the client side of Terse; the server's repository can be found [here](https://tangled.org/joshuaward.roomy.chat/Terse-server), and is not necessarily up-to-date with the client, as both are still in-dev, but keep an eye out!</sub>
<br>

Terse is a search engine directly in your terminal, designed to give you answers, cheatsheets, code snippets, etc., fast, easy, and bloat-free.

<!-- https://dev.to/asyraf/how-to-add-dropdown-in-markdown-o78 -->
<details>
<summary><h3>Why use Terse?</h3></summary>
<br>

<h4>Terse is fast</h4>
Terse gives you the answers to the questions you need most, right in your terminal. Written in Rust, as if it mattered.
Opening up a browser, searching, navigating results (and ads), rendering out HTML and <em>JavaScript</em>, even using a <em>mouse</em> is much slower, not to mention a ton more expensive.

<h4>Terse is lightweight</h4>
No HTML, no fancy graphics, no rendering engines, no React, no analytics, no ads, no images, no AI processing trillions upon trillions of tensor operations to generate each individual word of 'how to center a div'; nothing but the info ya need.

<h4>Terse is useful</h4>
Think about the number of Web searches you do each day. How many are for really basic stuff you just can't seem to remember? Now imagine if you could get those answers in half the time, and distraction-free, too. Terse does that.

</details>

<details>
<summary><h3>How does Terse work?</h3></summary>
<br>

<h4>Searching</h4>
Just type in a query; <code>trs center div</code> will work, or <code>trs rust result mapping functions</code> for something that would have been REALLY helpful in Terse's development.

Actually, before that, you'll need to-

<h4>Connecting to a server</h4>
Terse doesn't open a portal to the whole Web or do a Google query for its answers; instead, you connect to a dedicated Terse server to search on it;
<code>trs --server add https://the-url-of-a-terse-server.com</code>
You can search on the server now! 

Note that this architecture has at least one advantage over other common cheat-sheet systems such as the (amazing) [cht.sh](https://cht.sh), [tldr](https://tldr.sh), or basic man pages, in that the backend is moddable; I.E. a Terse server be a wrapper for these sources of cheat sheets, but these cheat sheets cannot wrap Terse.

<h4>Publishing</h4>
Some servers (hopefully) in the future will allow you to publish to them;
<code>trs --pub -t "Format String Cheatsheet - All Languages" -p docs/format-strings.txt</code>, for example.
This way you can access answers written by devs like you who had the same problem as you, and besides answers-

<h4>Code Snippets</h4>
You can publish anything to Terse! Terse supports easy downloading / copying of code files people have uploaded
(things that are so common yet everyone rewrites anew each time; think character controllers in Unity or a render loop in Pygame, etc etc etc)

</details>

<div>

<hr>

<sub>No part of Terse or Terse-server was written, designed, or influenced in any way by AI, except in spite. Terse is 100% human-made</sub>

</div>
