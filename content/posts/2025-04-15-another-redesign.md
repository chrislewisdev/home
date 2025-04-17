---
title = "Another website redesign"
description = "Indulging yet another desire to rebuild things from scratch"
---

Well, it's been a year and a half since my last site update, and about four years since I last updated the design! I was getting sick of the old website setup and didn't really enjoy working with it anymore, so naturally I stopped making updates altogether until I built up the willpower to rebuild the whole thing.

What was wrong with the old setup? Well, I guess it all came down to its reliance on [Jekyll](https://jekyllrb.com/). For context, this site is a purely static site built from [markdown](https://www.markdownguide.org/) files, and Jekyll was my site generator of choice for it. However, Jekyll is built in [Ruby](https://www.ruby-lang.org/en/), and running it involves having Ruby and its various dependencies installed on your system. I'm not a Ruby coder, so I've never ever used it except for this site. After encountering a few issues installing it on different machines over the years, I decided I just don't like the ecosystem altogether and want to be rid of it.

The question naturally follows: what then to use instead of Jekyll? If you've read some of my earlier blog posts, the answer probably isn't surprising: I decided to write my own site generator from scratch. Not the fastest approach, but since there is an abundance of markdown parsing libraries available for many languages, the hardest part - transforming markdown into HTML<sup>[1](#ref-1)</sup> - is already taken care of. What remains is mainly the job of choosing some appropriate conventions for folder structure, page layout etc, and you can have a working site generator in just a couple hundred lines of code. If you like, you can check out the [source code](https://github.com/chrislewisdev/home).

My language of choice for this rebuild was [Rust](https://www.rust-lang.org/), which is fast becoming my go-to language for hobby projects. Rust has a steep learning curve, but that's actually part of why I like it - after many years of programming, it's nice to be using a language that stretches my brain muscles in directions they haven't gone before. It grants me the satisfaction of approaching things from a low-level angle while affording many modern conveniences that I want in a language.

## A change in ethos

Structurally, the site hasn't changed much, but this rebuild does represent a bit of a change in perspective. When naming the git repo for it, I was originally going for some boring name like 'chrislewisdev-rs' signifying the Rust rewrite, but eventually I settled on 'home'. I want this website to be like a digital extension of my own home - somewhere a guest can come in, take a look around, and leave with a better understanding of me as a person. It may not be lavishly decorated, and I may not always have a lot to show, but it should at least be true to me.

It's interesting to note how this website has changed in focus over the years. Starting as a portfolio site while graduating uni, and acting as a digital resume when I looked to change jobs years later, the early years were definitely bent towards representing myself as a professional. Nowadays, it's far more important to me to represent myself simply as an individual, and while I still consider myself a career-focused person, that definitely reflects a shift in my priorities in the past decade.

## A change in deployment workflows

The deployment workflow for the Jekyll version of the site was kind of weird, probably driven by an experimental mindset at the time. Since this site is hosted on a Raspberry Pi, I configured a webhook in GitHub that would notify the Pi whenever new changes are pushed, which would trigger a build script to pull down the latest changes, build the site, and update all the served HTML files.

This workflow worked perfectly fine but it did require that the Pi have all the Jekyll dependencies such as Ruby installed on it. For this rewrite, I wanted to reduce my reliance on manual steps like that, so I updated the workflow to be far more sensible using GitHub actions, which builds the site and then deploys all the HTML files to the Pi using `rsync`.

## Future plans

So the site is now rewritten, and I can bring myself to make some blog updates again, but there are still some loose ends I'd like to clean up. My dynamic DNS is hosted in Netlify, a service that I barely use anymore, so I'd like to move all those DNS entries to AWS some day and just have a script/program that updates them using the AWS CLI or SDK. Once that's done, I could probably delete my Netlify account altogether and remove one more piece of digital cruft from my life.

For actual site content, I would like to do a better job of showing what's going on in my life, whether that's talking about new projects or just games/music I'm really enjoying right now. I'm not entirely sure of the best way to express that, since I've never been good at making regular updates, but hopefully I can find a way to inject a little more personality here.

&nbsp;

*<a name="ref-1">1</a> Well actually, I did once try to write my own markdown parser as well, which was fun for a bit but ultimately felt like way too much effort for something that has already been solved many times over*