---
layout: post
title: Gameboy Devlog&#58; Sprite Sizes
description: Talking about my learnings after running my Gameboy game on real hardware for the first time.
---

I've recently been working on a game for the original Gameboy in my spare time, and after waiting a while for my Everdrive to arrive I was finally able to see some of my early code running on real-deal hardware! This devlog is going to cover my learnings from running it on the Gameboy for the first time and what I'll be doing about it moving forward.

## Sprite Sizes

First things first: everything on the real Gameboy looks waaaaaay smaller than it does running on an emulator. My game is intended to be reasonably fast-paced and as such, the screen needs to be easily readable. So when I looked at my 16x16 pixel player sprite on the Gameboy, I was kinda disappointed to find that I basically had to squint to properly make it out. Although 16x16 is a pretty common sprite size for the Gameboy, I think for my game it needs to be bigger to feel right. Just check out this photo:

![](/assets/images/gb-devlog-sprites/16x16.jpg)

It might not be obvious there, but my game is going to be all about skateboarding, pulling off tricks and grinds to rack up points. So when a lot of the player's focus will need to be on their sprite to help time jumps and land tricks, having it take up such a small amount of the screen feels really weird!

Now, truth be told, I had kind of anticipated this might be a problem. So I had already prepared some versions of the game with a sprite size of 24x24 and 32x32. Here's how it looks with 24x24:

![](/assets/images/gb-devlog-sprites/24x24.jpg)

It's definitely better, but still feels kinda small and weird to me. So I went and tried out 32x32:

![](/assets/images/gb-devlog-sprites/32x32.jpg)

Honestly, 32x32 feels like it crosses a threshold into being too large. On one hand, the player sprite feels really nice and clear here, but because it's so large, all the rest of the game world needs to be scaled up too, and this means that the player can't see as much of the level on screen. This will give the player less time to react to upcoming obstacles, and might make it difficult to co-ordinate your tricks accordingly.

So, what size should I go with? Although I had some trouble deciding, for now I'm going to go ahead with the 32x32 size. Although it has its downsides, it just feels right to me, and my hope is that being able to see the sprite so clearly on the screen will make landing tricks that much more satisfying to watch. And hey, I can always experiment with some other slightly different sizes - eg 28x28 - in the future if I really need to.

## Motion Blur

The other problem that became evident on the Gameboy is that of the screen blurriness during motion. Have another look at the first photo again:

![](/assets/images/gb-devlog-sprites/16x16.jpg)

Notice how some of the background looks really faint and blurry? This is just the nature of the beast that is the original Gameboy's screen: the response time of the individual pixels is quite poor, so when things rapidly move across screen, you actually don't get a crystal clear image on each frame. Since my game is constantly scrolling a couple pixels left each frame, this makes the background really unclear.

Newer hardware such as the Gameboy Colour doesn't suffer from this problem, but I really want to make this game look as good as I can on the full original hardware. So, I intend to experiment with changing the game's frame rate to try and compensate for the blurry image. The Gameboy natively runs at 60 frames per second, but if I limit the game's screen updates to 30fps instead, that should at least make the visuals somewhat clearer. After all, updating the pixels half as often should help account for that slow response time the screen has! But it remains to be seen just how much of a difference that will make.