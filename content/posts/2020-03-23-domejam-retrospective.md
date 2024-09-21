---
layout: post
title: DOMEjam retrospective
description: Reflecting on a small game I developed for DOMEjam in 2020.
---

I don't often have time to participate in game jams, normally working on projects at my own slow pace, but when I recently saw [DOMEjam](https://itch.io/jam/domejam) announced it struck me as the perfect opportunity to try out a new game framework and learn a new programming language in a forgiving time limit (10 days - long enough that I don't need to rush, but short enough that it keeps me focused).

[DOME](https://domeengine.com/) is a little different from other game frameworks in that it is based on the [Wren](http://wren.io/) programming language, a lesser-used but seemingly quite capable scripting language when compared against popular mainstay [Lua](https://www.lua.org/). I have most recently been using [PICO-8](https://www.lexaloffle.com/pico-8.php) a lot so I was keen to see how DOME compares.

## Deciding what to make

In deciding what game to make, I wanted to keep the scope quite focused in order to fit within my schedule. While searching for inspiration, I kept thinking of how excited I am for the upcoming [Playdate handheld](https://play.date/), and really wanted to see if I could make something that could feasibly run on that console when it is released. One advantage of DOME compared to PICO-8 is that as a more generalised framework, it grants you more freedom with screen size and colour palette. So it was decided: in keeping with the hardware specs of Playdate, I would create a game that fits within 400x240 pixels and using only black & white colours, using DOME.

Ten days later, Clean That Castle! was complete.

![](/assets/images/ctc/screenshot01.png)

## Tools

Since DOME is not an all-inclusive editor for sprites and audio like PICO-8 is, I had to fall back on different tools for this game. I ended up using [GIMP](https://www.gimp.org/) to draw up some very basic sprites for the game (thankfully, I did not need many in the end), and used PICO-8's audio editor to create a couple of sound effects. I also used [VS Code](https://code.visualstudio.com/) as my text editor.

## The experience

I'll break down my experience using DOME to make this game into sections of dot points: the good, the bad, and other miscellaneous comments.

### The good

- Wren is a fairly easy programming language to pick up. The [documentation](http://wren.io/syntax.html) is quite straightforward and to the point, explaining the language's core strengths clearly
- Having first-class object-oriented programming features available with the ease-of-use of a scripting language is a very, very nice combination. Although languages like Lua do accommodate some form of object-orientation, it can feel quite clunky and confusing to work with. In Wren, structuring your game code using common OOP structures feels very natural.
- Coming from PICO-8, DOME's API feels quite intuitive (and I suspect DOME has deliberately taken inspiration from PICO-8 in order to achieve this). Common functions like drawing graphics, playing audio, and checking input are largely similar and in some cases are a little more powerful.
- DOME's API paired with Wren's ease-of-use does live up to the name of "the comfortable framework" that DOME touts on its website. Aside from the stress imposed by the jam's time limit, I did find working in it to be quite pleasant and relaxed.

### The bad

- Although DOME's API is fairly similar to PICO-8, there are some  features missing that PICO-8 users will take for granted. Things like in-built tilemap functions or easy saving of game data to disk aren't provided, and might cause some PICO-8 veterans to shy away.
- Wren appears to have some quirks that aren't clearly covered in the documentation - nothing major, but there nonetheless. For example, whitespace handling is a little inconsistent with what you might expect from other languages. Placing a newline before a method call seems to be valid only if the preceding dot is placed on the previous line; rather than allowing the dot to be placed on the new line right before the method call.
- DOME's youth means there are few examples available that you can use as a jumping-off point for writing your own code. Hopefully, as a result of the jam and as time goes by, this will improve as it sees more usage.
- DOME does not currently provide the ability to build your game for browsers, which in this day and age feels almost essential. Having to download the game to play it is not a deal-breaker, but I much prefer the accessibility factor of being able to direct people to a URL and have them play it right there.

### Other comments

- Playdate's screen specs, allowing only black & white colours (not even greys) is HARD to create art for. Of course, I'm a programmer, not an artist, so I'm sure many others will be able to make better use of its limitations than I have - but I very quickly realised that having no other colours at my disposal makes it quite difficult to communicate your game's design to the player.

## The results

I ended up creating a small puzzle game based around match-three mechanics, which you can play on [itch](https://magspinner.itch.io/clean-that-castle).

![](/assets/images/ctc/screenshot02.png)

As is typical for any game jam, I didn't end up having time to implement everything I wanted - I was hoping to give the game a stronger theme, some proper background music, and some more inviting art. My current aim is to make small, wholesome games, and this one is most definitely small but I feel like it needs more work to make it feel properly inviting and wholesome to play. In any case, I hope you can enjoy the small smattering of puzzles I created with it!

As for DOME itself, would I come back and use it for another project? I'm a little unsure. I definitely enjoyed writing a game in Wren, and DOME certainly provides a sufficiently capable feature set for making small 2D games. I could see myself happily using it again, but at the same time, I feel like I probably wouldn't prefer to use it over PICO-8, provided I wanted to make a game that fits within PICO-8's limitations.

Ultimately, I feel like DOME is a good tool to have at my disposal for future projects, so I'll definitely keep it in mind even if I don't use it again for some time. I do hope it gains a larger audience and continues to grow, so I will keep an eye on it and see how it evolves over time. I think with a bit more community following and perhaps some more features to set it apart from other frameworks, it could carve out its own niche in the game development community.