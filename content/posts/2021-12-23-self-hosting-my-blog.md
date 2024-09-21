---
title: Self-hosting my blog
description: This blog now proudly served from my home!
layout: post
---

As of this blog post going up, my website is now being served from my Raspberry Pi at home! This is in contrast to my previous static site hosting on [Netlify](https://www.netlify.com/), and before that on a [VentraIP](https://ventraip.com.au/) hosting plan.

## Why?

I've been doing some [reading](https://networked.substack.com/p/web3-i-have-my-daots) [about](https://adamd.mirror.xyz/tap-94pfnB_WvMT6W8odoYe5xiMD00WDFNouTytEHN0) [web3](https://modelcitizen.substack.com/p/is-crypto-bullshit) [lately](https://society.robinsloan.com/archive/notes-on-web3/), in order to understand what the hell people on the internet are talking about (I'm not sold on it, by the way, but that's a topic for another blog post that I may or may not bother to write). One of the main selling points of web3 seems to be a return to the pre-social-media days of the internet, granting users more agency and control over their online identity. If you've read my previous post ["In search of a more personal internet"](/posts/in-search-of-a-more-personal-internet), you'll know that I'm somewhat disenfranchised with the current state of the internet and social media, so this particular selling point does resonate quite well with me. I, however, do not believe that we require the blockchain to accomplish this, and felt inspired to instead take a little more control of my piece of the internet by self-hosting my blog.

This year for me has also been a lot about getting back to some of the "grass roots" of programming rather than dealing with things through countless layers of abstraction, such as learning how to code for the Gameboy, and tinkering around in Linux with vim. Turning my Raspberry Pi into a fully-fledged web server I think ties in quite nicely with that.

## The setup

Raspberry Pi as a web server is hardly a new concept, so there were plenty of resources to help guide me through getting it sorted. I'll go through a few different elements of the setup here to give you an idea of how it all hangs together.

### The site content

The website itself remains unchanged from its [Jekyll](https://jekyllrb.com/) incarnation, so I installed Ruby and Jekyll and all its dependencies on the Pi so that I can build all of the content directly on it (rather than deploying the code from elsewhere). One `git clone` and a build later, I could see that it was working correctly on my local network using `jekyll serve`.

### The web server

I only have very limited experience configuring web servers, but I knew that [nginx](https://nginx.org/) is a very popular option these days and it seemed like it was well-suited for lightweight setups like mine so I looked into setting that up as my server.

To my surprise, installing and configuring nginx was really simple to get started with: after a simple `apt-get install nginx`, it came pre-configured to serve basic html content from a directory. All I had to do was copy the site content into there and it was up and running!

I did want to experiment with customising the config just a little however, so I did make some tweaks in order to serve my site from a subdirectory (in case I wanted to host more than one site in the future), which looked something like this:

{% highlight nginx %}
{% raw %}
server {
    root /var/www/html/chrislewisdev;

    index index.html;

    server_name pi.chrislewisdev.com pi.chrislewis.dev;

    location / {
        try_files $uri $uri/ =404;
    }
}
{% endraw %}
{% endhighlight %}

### The DNS

Serving my content with nginx was all well and good, but it's not much use without a domain name pointing to my Pi. I wanted to use a subdomain of my main site, so `pi.chrislewis.dev`, while I test everything out and get it ready. Since my home router uses a standard dynamic IP address, I needed to set up some kind of dynamic DNS lest my IP get re-assigned and my domain ends up pointing to nowhere.

I've been using Netlify to manage my DNS records, and I wanted to keep it there in case I need to fallback to my old hosting at any point, but unfortunately Netlify doesn't provide any officially-supported dynamic DNS mechanism. Luckily, some kind soul on the internet must have had this same problem, because I found [netlify-dynamic-dns](https://github.com/oscartbeaumont/netlify-dynamic-dns) from [@oscartbeaumont](https://github.com/oscartbeaumont) that provides a tool that can be run to periodically update my Netlify DNS settings to point to my current IP address. Now, I could just run the following on startup, and it takes care of it all for me:

{% highlight sh %}
{% raw %}
nddns -accesstoken <REDACTED> -zone chrislewis.dev -record pi -interval 10 &
{% endraw %}
{% endhighlight %}

(In typical Linux fashion, there are several ways to run scripts on startup but not all of them are guaranteed to work depending on your setup and needs. I found the most success in this case by just calling my desired script inside my rc.local file)

#### Addendum

Ok, it turns out the above was partially a lie. Although `nddns` was just what I needed, it had one limitation in that it wouldn't let me update the *root* DNS record for a domain, which I needed when pointing the base `chrislewis.dev` domain to the Pi. To work around this, I was able to make some small tweaks to the code to provide that as an option, and will create a pull request on the main repo in case anyone else might have need of that in the future.

### HTTPS

With DNS configured, there was one last major blocker before I could serve my site under a .dev domain: HTTPS. All .dev domains are required to be served over HTTPS, so a plain HTTP connection wouldn't cut it in this case. To set up SSL certificates for nginx, I opted to use the popular certbot tool offered up by Let's Encrypt. This has the handy ability to not only generate your SSL certificates but also automatically configure your server to use them, and with my nginx setup this proved quite simple: all I had to was run `certbot --nginx` and follow the command prompts.

### Automated deployment

No web project of mine would be complete without a CI/CD workflow, since I'll be darned if I'm going to have to manually redeploy my website every time I update it. But rather than opt for a full CI workflow using one of the many many CI tools out there, I decided for the following process that will run purely on the Pi itself:

1. Add a webhook to my Github repository that hits `https://chrislewis.dev/hooks/build-chrislewisdev` whenever commits are pushed
2. Listen to the webhook and run a script whenever it is hit
3. The script pulls down the latest site code, runs a build, and then updates the server directory with the new files

Initially, I thought I would need to create some small bespoke server to handle the webhooks part of this process, but as it turns out there was yet another handy tool out there for me, this time the [webhook](https://github.com/adnanh/webhook) repo by [@adnanh](https://github.com/adnanh). This tool fits my exact use case: listening for webhooks, and running scripts in response. All I had to do was provide some fairly simple config, most of which are "extras" to ensure that the webhook will only be triggered by Github itself (rather than any old person hitting the URL) and that only pushes to master will trigger the build:

{% highlight yml %}
{% raw %}
- id: "build-chrislewisdev"
  execute-command: "~/builds/chrislewisdev.sh"
  command-working-directory: "~/code/chrislewisdev"
  trigger-rule:
    and:
      - match:
          type: "payload-hmac-sha256"
          secret: <REDACTED>
          parameter:
            source: "header"
            name: "X-Hub-Signature-256"
      - match:
          type: "value"
          value: "refs/heads/master"
          parameter:
            source: "payload"
            name: "ref"
{% endraw %}
{% endhighlight %}

*(to be honest, one day I might still create a bespoke solution for the webhook, for fun and/or learning)*

The build script itself is even simpler:

{% highlight sh %}
{% raw %}
#!/bin/sh
git pull
bundle exec jekyll build
sudo rsync -r --delete-after _site/ /var/www/html/chrislewisdev/
{% endraw %}
{% endhighlight %}

Since all it needs to do is pull the latest commits and build everything locally, the builds are damn fast too, arguably even faster than my old Netlify builds since there's less moving parts involved.

#### Addendum

This is perhaps not too surprising, but one *disadvantage* of a build process like this that relies on the server having all the code and dependencies to build it, is that it is somewhat fragile should the build environment change. While putting the site live, I noticed it failed to do a `git pull` a couple times because Ruby's `Gemfile.lock` had diverged from the main copy simply by way of running the build on a different machine. I am happy to accept this for the moment and consider tweaks I can make as I go that may add resilience while keeping things simple.

### Putting it live

With everything tested and working, all I had to do was tweak my existing config to use the primary chrislewis.dev domain. This meant a bit of downtime for my site since I couldn't set up the SSL certificates for that domain until it was actually pointing at my Pi, but that's not really a big deal.

## Next steps

With the sever up and running, it might be nice to set up some lightweight monitoring to understand how much traffic is coming in and how much CPU load it creates, but as of yet I haven't found anything that I really liked the look of. In a pinch I can just run `top` on the server to see how the CPU is going at any time. It might also be nice to have some kind of uptime monitoring so I can be notified if it ever goes down suddenly.

Moving my site in this fashion has also got me thinking about any cool extras I'd like to add, since now I could build just about anything (static or otherwise) and serve it up under the same umbrella as the main site content. This is still turning over in my head but I did discover the term ["digital garden"](https://maggieappleton.com/garden-history) just today, and it might be nice to incorporate something like that in the future.

## Conclusion

So there we go, everything is coming from my Pi now and I'm now just a little more digitally self-reliant! Of course, it remains to be seen how well it handles the traffic, but honestly I don't think this site gets enough traffic to pose any problem to it.

I'm not always great at connecting with people in the real world, but now whenever you visit my site, those ones and zeroes will be making a trip all the way from my home to you, and I think that's pretty neat.