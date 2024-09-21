---
layout: post
description: A small summary of lessons from developing a mobile game in Unity.
---

_This is a post I actually wrote up 2 years ago when I was finishing up development on Face Invaders as part of Electric Mammoth. It may not be very relevant by now, but it seemed like a waste to just have it sitting around all that time for no reason- so here it is!_

## Memory Budgeting

Regardless of whether you're trying to make a large 3D game or a simple 2D one, if you're developing on mobile, you should be super-duper conscious of your memory budget. Keep a close eye on your memory usage through Xcode or anything else that will help. 2D sprite sheets can start to really blow out in terms of texture size (even with the good packing implemented by plugins such as 2D Toolkit), and you don't want a surprise one day when your game crashes due to running out of memory.

Memory budgeting is something you should tackle early, though perhaps best to allow a short period of seeing how your usage is trending while you add more assets to the game. Otherwise you could end up facing hours upon hours of lost work for having to cut something, re-work it, or downright sacrificing quality to reduce memory usage. This is one we really should have seen coming, but didn't.

## Deadlines

Face Invaders was a project that was intended to take 6 months. As of writing this, we're very close to hitting 3 times that duration in development.

There are various reasons for this, the most important one easily being the time-commitment to the project. We currently all either study or work full-time, so there have been many periods where putting a lot of time into the game was simply not feasible. We also had no real deadlines for any parts of the game- which is largely due to the previous reason, but nonetheless did not help our progress, allowing us to take as long as we want on any feature.

By far our most productive periods have been when there is a deadline for us to aim for, such as a conference/convention to show the game at, or anything else which involved having the game come into contact with the outside world.

I'm not sure how much better we could have done in our situation, but it's obvious that having time-based goals to work towards helps to either increase productivity or ensure that your time is being spent on what really needs it most. A relaxed development cycle has its benefits, but it might be better to introduce even some small deadlines that encourage you to make efficient use of your time and avoid blowing out development time.

## Debug Shortcuts

Setting up appropriate debug features for use on a phone is difficult, since you can't resort to shortcut keys like on a PC. For Face Invaders, I resorted to simply having any shortcut keys available for use in the PC editor, with no substitutes available when running on a device. This is fine when most of your work takes place in the editor, but the more time you spend testing on a device the bigger the problem is going to be, especially if you find yourself having to tweak things in-editor and then re-deploy to the device, which becomes very costly on time per tweak.

With a relatively small amount of time and effort, it would make sense to set up a debug-specific GUI for use on the device to replace keyboard shortcuts. Restricted even to simple buttons (e.g. for spawning enemies, activating god mode etc), I could see it saving a lot of time- and depending on your chosen UI system, you may be able to add in various sliders/configuration elements with little extra effort.

What I would really love is a debug utility to allow you to send keypresses to a mobile device, and use this to trigger debug shortcuts while playtesting. There's no way to do this that I know of, but I'm sure it must be possible to make one.

## Configuration Profiles

I set out from the start to try make configuring the game's settings from within the editor easy, which didn't take a lot of effort thanks to Unity's auto-magic GUI editors. Thanks to this, debugging and making gameplay changes were all easier since I could quickly enable/disable certain things or tone down features either to help examine a bug, or to see how it affects the gameplay. However, when making changes is so easy, it's also very easy to forget what you had them set to in the first place, so if you just changed some finely-tuned gameplay values for your debugging, you might end up having to try find them all over again.

There are some obvious common-sense solutions to this- write them down, do your testing in a separate game scene, and probably more- but it's also made me wonder if there'd be some benefit to having a simple system that uses configuration ÔÇÿprofiles'. That is to say, where you can create and manage separate sets of configuration settings for the game, each for a particular purpose such as the core gameplay or debug modes. Aside from making it easier to manage and switch between desired configurations in the editor, maybe they could even be utilised at runtime for playtesting and debugging on the fly- such as switching between similar but different sets of gameplay configurations to see how they each contrast with each other.

## Know Your Tools

This one's a simple one, but every now and then I'd end up implementing things myself when I could have used something else that Unity already provides. I spent a couple days working on and tweaking a sub-par system for our touch-trail effect, when I could have just used a Unity Trail Renderer. To amend this, I should obviously try be more aware of the features in the engine I'm using, and before implementing anything sizable, ask myself if there's no pre-packaged option that suits our needs.

## Anticipate Change

This is something I learn from almost every project I work on, in that it comes back to bite me in new and unexpected ways. In any medium-to-long-term project, the design will always change from what you started with, from expanding systems to completely re-working them. Maybe a designer will suggest one small extra feature which will throw a big spanner in the works. Don't design a system under a spoken contract that it won't change, because it will. Or at least, if it's something that you make a quick-and-dirty implementation for, be prepared to throw that away when it does change.

When we begun work on our full UI, I opted to develop our UI systems myself rather than use a package such as nGUI since I expected the UI to only need to behave in certain ways. As the UI design expanded, so did the UI system in ways that were not exactly elegant. It worked, and my implementation was able to survive without becoming a complete disaster, but it also would have been considerably faster if I'd anticipated a change for larger scope in our UI and chosen to use a UI package instead.
