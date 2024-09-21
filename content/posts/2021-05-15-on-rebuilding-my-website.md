---
layout: post
title: On Rebuilding My Website
description: Discussing my motivations behind the latest rebuild of my site as well as my choice of tools for this iteration.
---

Like most developers, every few years or so I'll decide it's high time to rebuild my personal website and give it a fresh new look. I thought I'd take a moment to talk about my motivations behind this current iteration of my site and my choice of tooling for the new build.

## Motivation

Truth be told, I first started thinking about rebuilding my website roughly 2 years before the time of writing this post, but it was only until a couple of months ago that I found the motivation to actually work on it in earnest.

So, what finally tipped me over the edge? I think it happened when I started researching how to develop games for the Gameboy; in search of decent tutorials and documentation, I had to trawl through many resources developed by hobbyists, many of which were hosted on their personal websites, some of them quite old. Browsing through all of these websites made me nostalgic for the times before most of what we consume on the internet became centralised to just a couple of social media apps; a time where having an online presence felt more unique and personal than just another Twitter profile.

At that time, my site was basically just an About Me page with links off to all my relevant profiles, and highlighting a couple pieces of work that I'm proud of. I decided I want to change that, and make my website something that felt more like a reflection of me.

## Design/Content

Just because I wanted my site to be a little more unique didn't mean I wanted to do anything super crazy. All told, I still wanted my site to be quite simple. But rather than being a single page targeted more towards potential employers, it would focus more on the things important to me.

I'd bring blogging back to my personal website, not because I intend to do any serious blogging again but because I'd like all my existing posts to be visible in one place alongside my other site content, rather than purely published on Medium. Additionally, self-hosting my blog feels more freeing than publishing through Medium, where I feel there are certain expectations and standards of writing. Here, I can feel free to post more journal-style posts that may even be more for my benefit rather than anyone else's.

I'd also expand the portfolio on my site to cover not just recent and noteworthy works of mine, but really anything in my history (development-related or otherwise) that I'm remotely proud of. Although I doubt anyone is particularly interested in reading about some old broken game projects from my past, it feels important to me to not just show what I am proud of now, but also some of the stepping stones that led me towards here. I also believe that by taking the time to list out some of my old projects, it helps me appreciate everything I've done in the past, rather than forgetting about them entirely.

## Tools

In keeping with the fairly simple design of my site, I wanted to keep things quite simple from the tech perspective as well. I spend enough time working on production web systems at work that I don't feel any need to over-engineer my personal website. So - surprise, surprise - I opted to use a static site generator this time around. Although static site generators have become quite popular in the last decade, I actually never had cause to properly use any of them until now, so even though the process ought to be quite simple, there would still be a new thing or two for me to learn.

Of course, these days there are a lot of static site generators out there. So which one did I use? Allow me to list the options I initially looked into and why I did or didn't use them:

### [Hugo](https://gohugo.io/)

Hugo appears to be one of the most popular and fully-featured static site generators these days, so when I decided to make a start on my site I installed this one and had a read of the docs to understand how to get started.

Although the getting started process for Hugo was quite simple, I ran into troubles when I started trying to work out how to actually build out a custom theme for my website. As soon as I started reading the docs for this, the process became a lot more complicated and hard to follow than I was expecting. Perhaps I missed something, but to my eye the Hugo documentation was quite large and made it difficult to find the info on what exactly I wanted to do, unless I first understood a lot of detail about how Hugo works. This gave me the feeling that although Hugo seemed quite powerful, it was not as user-friendly as I had hoped, so I set off in search of other alternatives.

### [NextJS](https://nextjs.org/)

Another one of the most popular tools these days, NextJS looked very modern, as well as having a very flashy getting started guide. Although it looked quite attractive, I was very quickly turned off this for a very simple reason: waaaaaaay too much Javascript.

Now, don't get me wrong: I like Javascript. I do a lot of React coding in my work and find it very enjoyable to use. But in wanting to keep my website quite simple, I became very attached to the idea of serving up my pages in the plainest HTML I can manage - so the idea of having the entire site built in React and all of the client-side overhead that comes with that was a huge turn-off for me.

### [Jekyll](https://jekyllrb.com/)

A somewhat older but still very common tool, I actually initially ignored Jekyll because I got the impression that it was an overall older and more limited product compared to the other options out there. But upon further inspection, I found that Jekyll was quite well-suited to my needs. Sporting a fairly straightforward organisation of HTML and Markdown files, with simple but sufficiently powerful templating features, I was able to get started building a custom design quite quickly and without needing to read a bunch of documentation just to understand how everything fits together. After a fairly quick read of their getting started guide, and a little bit of tinkering on my end, I felt like I was already much closer to my goal than I had been when trying out the previous options.

## Implementation

Using Jekyll, the implementation process was mostly as straightforward as building out the HTML layouts and markdown files for my content. I migrated all of my existing Medium posts into Markdown format using the [mediumexporter tool](https://macropus.medium.com/export-your-medium-posts-to-markdown-b5ccc8cb0050), but each post still had various formatting errors that I needed to fix in order for them to look correct, and I also needed to manually re-download all the images I used so they can be hosted locally (thankfully there were not too many, otherwise I probably would have needed to find some automated way to do this).

I did have one funny moment when designing my home page actually: since I wanted to display a random showcase item on each load of the home page, I first reached for Jekyll's inbuilt `random` filter. But of course - since it's a static site - the entry was only randomised at build time and stayed the same from then on! In order to *actually* present a random item each time, I had no choice but to resort to Javascript. Ultimately, I needed to use Jekyll's templaying syntax to render out a Javascript array of each showcase entry as part of the page, and then insert it into the page after pageload, like so:

{% highlight html %}
{% raw %}
<script type="text/javascript">
var showcaseItems = [
{% for item in site.showcase_items %}
{
    title: `{{ item.title }}`,
    content: `{{ item.content }}`
},
{% endfor %}
];

window.addEventListener('DOMContentLoaded', function() {
    var spotlightItem = showcaseItems[Math.floor(Math.random() * showcaseItems.length)];
    document.getElementById('spotlight').innerHTML = `<h2>Random spotlight: ${spotlightItem.title}</h2><p>${spotlightItem.content}</p>`;
});
</script>
{% endraw %}
{% endhighlight %}

It's not exactly the prettiest solution, but it seems to work perfectly well.

## Finishing up

For hosting, I'm using [Netlify's](https://www.netlify.com/) free static site hosting, deployed straight from the Github repo. I used to use a full webhost setup from [VentraIP](https://ventraip.com.au/), but after realising I no longer had any need for a full webhost I figured it would be a nice cost-saving to get rid of that and use one of the myriad free hosting services available these days.

All in all, I'm pretty happy with how my site's looking now, however far from perfect it may be. At the very least, it feels more reflective of who I am right now, and it pleases me to know that anyone that may happen to visit is getting some small glimpse into that. After all, I feel like everything we put out into the world reflects something personal about us, and for someone to witness that and appreciate it on any level is akin to making a small personal connection.