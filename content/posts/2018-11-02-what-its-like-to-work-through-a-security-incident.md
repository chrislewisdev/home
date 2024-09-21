---
layout: post
title: What It's Like To Work Through A Security Incident
description: Recounting my experience of dealing with a security incident at my workplace
---

![](/assets/images/security/alert.png)

Earlier this year, the company I work for as a software developer suffered a security incident. What followed was a very unique few months that I'll remember for the rest of my career.

Prior to the incident, I'd had no real clue what to expect if one were to happen. There seem to be few first-hand accounts of what it's like to be in that kind of situation published online. I want to share what my experience was like during this time, so that you might have at least some idea of what it would be like if your company were to experience the same.

***Disclaimer**: The following is a story about my personal experience of working through a security incident as a developer. Security is not my specialty; I was not at the "front lines" of the event, and I won't be divulging any technical details of it here. Depending on your role and the company you work for, circumstances could be entirely different for you, but I hope you'll find value in it all the same!*

## Initial Discovery

Around late May, I was working from home and saw a company email go around stating that some suspicious activity had been discovered in one area of the system, and it had been quarantined while investigation was ongoing. I was pretty intrigued by what exactly had happened, so on my next day in the office, I asked around about it. The impression I got was of a fairly minor infraction; something that had been dealt with before there was even the potential for any harm. It sounded like everything was done and dusted, so after chatting about it for a little, I went straight back to work like normal.

Just an hour or two later, I got a tap on the shoulder from one of our leads and was asked to hop into a meeting that was in progress. I wasn't sure what he wanted but I was happy to join. When I did, I walked into a room containing several other leads, a few developers, and our entire security team. I was quickly briefed that everything being discussed was of a "need to know" nature and not to be casually spread around the office. Ultimately, it was said that the potential impact of the incident was more widespread than initially thought. Although there was no longer an active threat, we were in a position where we had to audit much of our system for any anomalies or signs of intrusion. Additionally, in light of the incident we had to re-assess any current security risks and plan out solutions to ensure this couldn't happen again.

I was taken aback by what I'd been told - when just earlier I'd thought that there was nothing at all to be worried about - but I must admit that it also felt good to be included in something confidential because my skills would be useful. In particular, I volunteered to be part of a group of people that would perform the auditing tasks, since my past projects had given me a good understanding of the software infrastructure that we would be inspecting.

Once the meeting was over, I essentially dropped all of my other tasks and joined the ad-hoc auditing team that we had just formed.

## In The Thick Of It

The next week or two was an interesting, confusing and somewhat frustrating time for several reasons.

Although our auditing work was very important, it was not exactly the most engaging. Auditing such a large amount of our infrastructure was a new task for us, and although we were seeking out tools to help us with it, we undertook a lot of it manually. Essentially, we would spend hours at a time clicking through resources to look for anything suspicious, and confirm there's nothing wrong. Finding nothing is a good thing, but also becomes very boring and draining quite quickly. It's also difficult to feel like you're adding value when doing repetitive tasks like these that don't have much of an outcome. Maybe it was just me, but morale in the team felt a bit low as we made our way through this task.

![](/assets/images/security/word-of-mouth.png)

Being mindful of word of mouth throughout the office was a little awkward too. While I was working away in the auditing team, the amount of work that was coming out of our response to the incident continued to grow. As the work grew, more developers needed to be brought in to work on it, and so the "need to know" information slowly spread further and further. Eventually, nearly every single developer in the company was working on something related to the incident. At this point, it becomes difficult to know how much any one person knows about everything that is going on. If I hear some new information from Developer A, should I be telling that to Developer B? Is the information that I'm privy to still as confidential as it was when I first found out? Although I fully trusted my coworkers, dealing with confidential information remains a little uncomfortable when it's not clear who should be knowing what.

Additionally, working with the auditing team meant that I was isolated from my normal development team. In fact, just a week before the incident, I had moved into a new team which had all the other members stationed internationally. This made the situation even more difficult because, for data sovereignty and contractual reasons, our international team did not have the same level of access to our systems, meaning that they could not be involved in the response efforts and hence were not informed of all the details of what was going on. This had the upside of meaning that they could continue with their planned work, with the downside of them being a little left in the dark during our response. It felt very detrimental to the forming of my new team to have me pulled away into some project that I couldn't really talk too much about and that required all of my attention. As the only local member of that team, I actually felt somewhat guilty that I didn't do more to try and keep them in the loop of what was happening.

## Stabilising Operations

After the first couple of weeks, our development teams had addressed most of the immediate risks and concerns, and the urgency around the situation was slowly decreasing. At this point, we were definitely shifting out of "fire fighting" mode to be starting work on some newly planned projects that were allocated a high priority after the incident. The auditing team that I was part of was running out of things to do, so its members were starting to move off into other teams to work on these new projects.

The project team that I moved into was given the opportunity to replace a fairly old but stable piece of our system - the kind that you would probably never touch under normal business circumstances because it "just works". Our task was to improve security of the system by replacing it with a brand new implementation of our choosing. Compared to the auditing work I was previously doing, this was a total breath of fresh air. We were able to rapidly experiment with new tech, narrow it down to the solution we thought best, and then refine it through to delivery in production - and the results we saw were really, really satisfying.

Honestly, working on this new project brought me to consider some of the "upsides" of what we were experiencing as a company. Suddenly, projects that would normally never get prioritised due to a lack of urgency were being worked on all over the place. As a developer, it was quite exciting to have the chance to work on these, even if the circumstances behind them were less than fortunate.

Around this point, the company was also looking to complete a third party verification to provide reassurance that there was no longer any active threat inside our system. Up until this is complete, your perspective on your software infrastructure and resources changes subtly. When you see something you don't recognise, or something that isn't clearly named with a clear purpose, you start to wonder if it could not be some innocuous resource left lying around and instead be a security risk. Once or twice I'd see a group of coworkers talking hurriedly amongst themselves about the discovery of a potentially suspicious resource, only to discover after a tense hour or two that it was something completely harmless. In this way, suffering a security incident can really make you start to jump at shadows and treat your system with paranoia.

## News & Social Media

After the incident was publicly announced, it didn't take long for news outlets to start reporting on it. Several people in the company suggested not reading any of the coverage, but I - and I wager many of my colleagues - read them anyway.

The news stories went exactly as I expected: sensationalist headlines cherry-picking alarming quotes from our public announcement were the norm. Even though I wasn't surprised by their content, I couldn't help but take some level of personal offence at the way the stories were written. Every day I could see the situation slowly improving as my coworkers gave their best efforts to set things right again; meanwhile, articles would try to spell out doom for the company or exaggerate the impact of the incident itself. After reading a number of these articles, I did start reading less and less of them, as there wasn't much point to me reading inflammatory articles anymore.

![](/assets/images/security/social-media.png)

Social media was a similar story. Out of some morbid curiosity, I would search for mentions of the company on Twitter to see what people were saying. When you work at a company which is typically not in the public eye, it feels very strange seeing people talk about it. There were a fair few angry and upset tweets about the incident, and when I saw people touting inaccurate information, it was tempting to chime in and correct them. Obviously, doing that would have been a terrible idea - getting defensive on a public forum could invite all sorts of unwanted attention and achieve nothing useful for myself or the company. Besides, I could definitely understand why people would be angry about what happened, especially at a time when there was limited information for people to really understand what impact the incident could have on them.

## Industry Peers

The response from media outlets made me a little apprehensive about how the incident would be viewed by other developers in local industry circles. If I attended or spoke at a meetup and told people where I work, what kind of response would I get? Would others' perception of me be coloured by what they'd heard about the company I work for?

I didn't want to let this faze me too much, so I continued to introduce myself and the company that I work for just like normal' which had some interesting results! I quickly learned that whenever I told someone where I worked, it would only take a moment to work out whether or not they had heard about the incident. Those who had heard would exhibit a certain wide-eyed expression (for just a brief moment) as they linked the name of the company back to their memory of whatever they had read about us. To be perfectly honest, it was quite entertaining! Whenever I noticed this, I couldn't help but acknowledge it with a laugh.

More importantly however, the conversations that ensued were always encouraging. I immediately got the impression that my peers in the industry were very understanding of the situation that we were in, and were often conscious that with enough bad luck, it could just as easily have happened to them instead. I think our incident is a cautionary tale to a lot of companies, reminding them that they need to take their security very seriously, and it was very heartening to see that everyone was so supportive about what we were going through. On occasion, some of my professional network would reach out to me asking how things were going and wishing us well, which was much appreciated.

## Ongoing Impact

All in all, it took a few months for things to really start feeling like they were going back to normal. Several months after the incident, it feels like we're back into a fairly normal rhythm of "business as usual", but you can still see its impact in almost everything we do.

There's still a lot of projects (both large and small) that need to be undertaken as a result of what happened, which has a huge impact on development roadmaps. Projects that were stopped in their tracks when the incident hit might not be able to be resumed, or do so only with considerable revisions to their scope and priority. There's still lots of exciting work to be done, but it can be a little sad seeing a project that you thought was going really well get upturned by the change in priorities.

One must also consider the financial impact to the company. Although I don't know any exact details, it's quite clear that both the upfront and ongoing costs of an event like this are incredible. Despite this, I haven't actually noticed anything different in the company from a financial perspective myself, which is comforting - though I definitely feel for those responsible for crunching the numbers and organising budgets for the foreseeable future.

## Final Reflections

Experiencing a security incident first-hand was equal measures stressful, confusing, and exciting. Although it was tough going for everyone in the company, it also allowed us to showcase some of the qualities that I'm really proud of in our workplace:

* No-blame culture. At no point was there even any question of who was "responsible" for the incident or the events that lead to it. Every single developer felt a shared ownership of the problem and what was required to fix it.

* Team agility. Dealing with the security incident entailed a breakdown of the normal lines of communication, with new teams rapidly forming around the problems to be solved, and developers from all areas joining in to assist where their skills were most needed. Although it was hectic at times, everyone was able to take it in their stride and work well together.

To me this all exemplifies the high level of trust we place in all of our developers. It sounds a little selfish to say, but in a way, I'm glad that this all happened where I could see it unfold first-hand and witness how well we could work together under such trying circumstances.

Prior to writing this, I was not able to find many similar stories about such developer experiences online, presumably because any information about security incidents is typically quite confidential and difficult to share. I hope that moving forward we can be more open about experiences like these as a community, as security incidents will continue to happen to more companies and they will all have a considerable impact on their employees.

Finally, I hope that reading about my experience can be of some help to you in your career. Although I wouldn't wish an event like this upon anyone, know that it can still hold a lot of positives for you as a developer, and can be a great learning experience in and of itself.

![](/assets/images/security/secure.png)

*Thanks for reading. It honestly means a lot to see anyone getting value out of whatever I contribute to the developer community.*

*Many thanks to [unDraw](https://undraw.co/) for the awesome free images used in this post.*
