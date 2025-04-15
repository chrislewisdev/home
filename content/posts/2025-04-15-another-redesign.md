---
title = "Another website redesign"
description = "Discussing the latest tech overhaul of this site!"
---

Well, it's been a year and a half since my last site update, and about four years since I last updated the design! To be honest I was starting to get sick of the old website setup and didn't really enjoy working with it anymore, so naturally I stopped making updates altogether until I built up the willpower to rebuild the whole thing.

What was wrong with the old setup? Well, I guess it all came down to its reliance on [Jekyll](https://jekyllrb.com/). For context, this site is a purely static site built from [markdown](https://www.markdownguide.org/) files, and Jekyll is one of many site generators available that are capable of doing this job. However, Jekyll is built in [Ruby](https://www.ruby-lang.org/en/), and running it involves having Ruby and its various dependencies installed on your system. I am not a Ruby user, and so the only time I would ever install it would be for working on my site. After jumping between a few different machines and sometimes fighting Ruby version mismatches, I decided I just don't like the ecosystem altogether and don't want to be reliant on it.

The question naturally follows: what to use instead of Jekyll then? If you've read some of my earlier blog posts, the answer probably won't be surprising: I decided to write my own site generator from scratch. Perhaps not the fastest or most practical choice, but there is fortunately an abundance of markdown parsing libraries available for many languages, so the hardest part of transforming markdown into HTML<sup>[1](#ref-1)</sup> is taken care of for you. What remains is mainly the job of choosing some appropriate conventions for folder structure, page layout etc, and you can have a working site generator in just a couple hundred lines of code.

*Discuss use of Rust*

## A change in ethos

## A change in deployment workflows

*Discuss future tech plans*

*<a name="ref-1">1</a> Well actually, I did once try to write my own markdown parser as well, which was fun for a bit but ultimately felt like way too much effort for something that has already been solved many times over*