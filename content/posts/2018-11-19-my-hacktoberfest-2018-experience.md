---
layout: post
title: My Hacktoberfest 2018 Experience
description: A post about my first time participating in Hacktoberfest.
---

![](/assets/images/hacktoberfest/logo.png)

This year I participated in my first ever [Hacktoberfest](https://hacktoberfest.digitalocean.com/). Hacktoberfest is a yearly event run in October by DigitalOcean that presents developers with a simple challenge: create 5 pull requests to open-source projects on GitHub, and get a free t-shirt!

As a developer with a few solid years of experience but barely any open-source contributions under my belt, I decided that this year I would finally have a crack at the challenge and force myself to get deeper into the world of open-source.

**Why?** In my time as a developer, I've already taken advantage of a great many open-source projects to help with my work or professional development. It feels right to give something back to open-source when I've gotten so much out of it already. However, making first-time contributions can be quite intimidating, which is why I hadn't really done any before now - and Hacktoberfest gave me the perfect excuse to finally break that barrier.

## My Goals

Before October began, I decided that if I was to make the most of the opportunity that Hacktoberfest presents, I should set some goals for myself beyond just the basics.

Hacktoberfest's audience spans across all skill levels of developers, and as such they place very few requirements on what you need to do to complete the challenge. For people who are new to software development, learning about pull request workflows by making 5 small documentation updates is a totally valid and rewarding way to participate. At my level of experience, however, I knew that I should push myself further to make sure I am actually learning new things during Hacktoberfest, rather than just participating in order to get a free shirt. For me, the shirt is just enough incentive to reward me for getting out of my comfort zone and meeting most of the goals that I set out for the month.

The goals that I ended up with for my Hacktoberfest were to complete:

* one documentation update

* one on a Microsoft project (which would net me [another free shirt](https://open.microsoft.com/2018/09/30/join-hacktoberfest-2018-celebration-microsoft/))

* one on a language I don't normally work in (i.e. not C#)

* one to address an existing GitHub issue

* one on a development tool that I use

I figured this would be enough to encourage me to make a variety of contributions throughout the month and maximise my learning.

## Finding Projects

When October 1st came, I signed up for Hacktoberfest and immediately started looking for projects to contribute to. I knew that my time to actually work on my pull requests alongside my day job would be limited, so I figured I could note down a variety of projects for me to come back to later on.

I found this process actually fairly challenging. At first I was browsing the long list of GitHub Issues with the ÔÇÿhacktoberfest' tag, but I found that many of these were either too simple for me, or in language that I wasn't interested in practicing, or just projects that I wasn't interested in contributing to. Instead, the most effective way for me to find projects to contribute to was a combination of GitHub's [Discover](https://github.com/discover) page (which gives you a list of suggested repos based on your history) and looking at the dev tools that I normally use to see which I could possibly contribute to. By the time I was done, I had at least a couple of projects listed that looked good to me, but I did keep looking throughout the month.

## My Contributions

By the time Hacktoberfest was over, I had made *exactly* five pull requests and completed the challenge. In order, those were on the following projects:

### [#1](https://github.com/viatsko/awesome-vscode/pull/145): awesome-vscode

A list of cool/useful extensions for Visual Studio Code that I found through GitHub's Discover page. An extension I really liked wasn't yet listed on the page, so I whipped up a PR to add it in. This was a nice, easy starter contribution, and (technically) ticked off my goal for a documentation update!

### [#2](https://github.com/terraform-providers/terraform-provider-aws/pull/6103): terraform-provider-aws

I use Terraform quite a bit, so the idea of contributing back to it sounded perfect for my Hacktoberfest plans. Thankfully, the Terraform AWS had lots of ÔÇÿgood first issues' tagged that were suitable for me to pick up. Since all the code is in Go, it was also a good chance to reinforce the small amount of Go knowledge I already had.

Interestingly enough, this contribution really ticked off multiple goals in one: I addressed a GitHub Issue on a tool that I use in a language that I don't normally use! It almost felt like cheating!

### [#3](https://github.com/wyze/vscode-hybrid-next/pull/3) and [#4](https://github.com/wyze/vscode-hybrid-next/pull/5): vscode-hybrid-next

Hybrid Next for VS Code has been my favourite editor theme for a while now, but there were some cases where the syntax highlighting for C# wasn't fully correct. It wasn't until Hacktoberfest that I actually realised that I could just contribute back to fix those edge cases that I discovered!

### [#5](https://github.com/Microsoft/vscode-go/pull/2051): vscode-go

As the month was nearing its end, I still hadn't yet made a contribution to a Microsoft repo. I really wanted to contribute to the core VS Code repo, but it was so popular that every good Issue I found had already been snapped up by someone! So instead, I had a look at the Microsoft-developed Go extension for VS Code, which was still quite active but not *too* active for me to find something to work on.

## What I Learned

Although not all of my pull requests were large, each one offered at least some kind of valuable learning, and there are a couple of parts that stick out to me from the experience.

With each contribution I made, it honestly felt really rewarding to be giving something back to the projects in question. Not only did I feel the accomplishment of having put in the time to put the change together, but I had the satisfaction of knowing that this could benefit many many more people than just myself.

Working on a variety of projects also gave me some really welcome exposure to new development experiences. My Terraform contribution taught me a little bit more about its inner workings, as well as teaching me more about developing in Go. Working on the Hybrid Next theme showed me what it's like to develop a custom theme for VS Code, and working on the Go extension gave me even more insight into VS Code's extension development while exposing me to a little bit more TypeScript.

Most importantly, I realised that contributing to open-source projects really doesn't have to be as scary as it might seem. The maintainers that I interacted with were friendly and helpful. Even the process of jumping into a totally new codebase was not as difficult as I had imagined, especially when you can look at other people's pull requests as a rough guide for what you might need to do in yours.

All in all, I'm really glad I decided to participate in Hacktoberfest this year. Although I'm not sure when I'll next make any open-source contributions, I can move forward knowing that if I see any more projects that I would like to work on, I can do so with a lot more confidence than I had before (and with two new shirts!).

*Thanks for reading! If you enjoyed this, maybe give me a follow over on [Twitter](https://twitter.com/chrislewisdev).*
