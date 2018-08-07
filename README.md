# The Glass Oak

This is a roguelike made in rust by following [Tomas Sedovic's Roguelike in Rust tutorial](https://tomassedovic.github.io/roguelike-tutorial/). As a reason for me to install Rust and to learn a bit about it, I'm following along with [/r/roguelikedev](https://www.reddit.com/r/roguelikedev/) in a [group effort](https://www.reddit.com/r/roguelikedev/comments/8ql895/roguelikedev_does_the_complete_roguelike_tutorial/) to work on the roguelike tutorial around the same time. We'll see if anything in this is useful enough to make its way over to my other project, Creek.

### What's Creek?

Fancy you should ask! It's a (very old-style vanilla js) simulation/game engine thing I wrote for the heck of it. You can check it out over at [its github](https://github.com/wcarss/creek), or see a small demo of it running [an action rpg](https://wcarss.ca/jabiru) or [tetris](https://wcarss.ca/tetris).

It's very much a work in progress!

### Is this good code? Can I learn from this or make money from it?

Haha, no, probably not.

### Can I try anyway / use it somewhere though?

Yeah! Like Creek, this work is published under the MIT License. You can check out the specifics of what that entails in the License file. Don't worry about contacting me to use anything here, but if you have questions or want to anyway, hit me up at [wyatt@wcarss.ca](mailto:wyatt@wcarss.ca)

### Dev Log
#### Dawn of The First Day: Wednesday, June 27, 2018
I'm starting this venture ~2 weeks into the community effort, but I'm pretty sure I'll be able to catch up now that I actually have Rust installed and running code.

I was up until 4-5 AM last night trying to install Rust on Windows 7! I fell asleep multiple times while waiting for things to install or to uninstall, but it finally worked at one point and I excitedly rushed through the [first part of the tutorial](https://tomassedovic.github.io/roguelike-tutorial/part-1-graphics.html). 

(If you're installing Rust for the first time on Windows, I wrote a [blog post about this](https://wcarss.ca/log/2018/06/installing-rust-on-windows-in-2018/) to help you out. My biggest problem was trying to be clever.)

As far as using Rust goes: the package system is already something I'm happy with, and the match function and some of the small syntactical things around while/if statements have struck me as cool so far -- I'm a little confused about why some tokens don't take semicolons (e.g. false or break), but I figure I'll pick it up as I go along. It's very surface stuff so far.

I tried to find a better font, but it seems like libtcod has some pretty exact (and unusual) requirements about the layout, format, etc. of its loaded font, and does not accept TTF files. I ended up mashing the Arial font Tomas Sedovic provides in [tcod-rs](https://github.com/tomassedovic/tcod-rs) together with a font called [Square](http://strlen.com/square/) by Wouten Van Oortmersen, after converting Oortmersen's supplied TTF into a png using [The DataBeaver's ttf2png ](http://www.tdb.fi/ttf2png.shtml), which only took a little bit of fiddling to make compile on ubuntu. But it *also* did a pretty weird job of laying the font out, so I'm not likely to do too much more tinkering with fonts for the near future.

Next up: [Part 2 of the tutorial](https://tomassedovic.github.io/roguelike-tutorial/part-2-object-map.html).

#### Some weeks later: August 7th, 2018
I've feasted and famined on this project and totally failed to update the development log as I went. I just completed Part 11, "Dungeon levels and character progression", and the character can now progress through dungeons and level up, in addition to all the menu systems, items, controls, saving & loading, fighting monsters, and all the other things that previous lessons have covered. There are only 2 lessons left -- improving the monster progression and some basic equipment functionality. Hope to get to them soon!
