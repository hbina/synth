---
title: Building a startup with Rust
author: Christos Hadjiaslanis
author_title: Founder
author_url: https://github.com/getsynth
author_image_url: https://avatars.githubusercontent.com/u/14791384?s=460&v=4
tags: [rust, startup]
description: This blog post is a compilation of thoughts around building a company with Rust
image: https://storage.googleapis.com/getsynth-public/media/rust.jpeg
hide_table_of_contents: false
---

![Rust](media/rust.jpeg)

When building a company you are setting out to fundamentally solve a problem.
For this reason, engineers have been systematically attracted by this romantic
idea of changing the world with your brain and a laptop. We are at heart problem
solvers.

As engineers, we can (and most of us have) become zealous at times about our
solutions to these problems. We have pragmatists who just get stuff done - they
address the symptom fast and effectively. We have idealists who will grind at an
elegant scalable solution and try to treat the disease. Whichever camp you
subscribe to, at a certain point you need to form an opinion about which
technologies you are going to use to solve the problems you see in the world -
and this opinion will inevitably cause contention.

Conventional wisdom is to 'use the right tool for the job'. The choice of
programming language for example, depends on the domain of the problem you are
trying to solve. If you're implementing some algorithm, in a secluded project,
it's easy to make the case about what the language for the job may be. You can
run a benchmark and literally test the execution time for each candidate
language (if you're optimising for execution time). You can persuade yourself
you've made a rational and 'objectively correct' decision.

However, in the context of building a business, your optimisation function is a
high-dimensional mess involving performance, development velocity, hiring,
server costs, ecosystem, tooling, support, licenses etc. You can assign weights
to what is most important for your business, but at the end of the day the
decision is inevitably qualitative.

At Synth, we're working on building the best data generator in the world. We
made a conscious decision to use Rust for our main line of products. After more
than a year of building I've had the opportunity to see Rust at its best and
worst in the context of starting a company - this post is a compilation of
these (at times cynical) thoughts.

## Development Velocity

Rust has a *really* steep learning curve. Coming from an OO background it took
me *months* to become productive in Rust. This was incredibly frustrating for me
as I felt that my lack of productivity was impacting the team, which it was.
Even when you eventually do become productive (and you will), Rust forces you to
really think deeply about what you're doing and things inevitably take longer to
get over the line. A poorly thought out design decision today can come back to
haunt you months later. What should be a simple change or refactor can end up
resulting in complete tear down as you try to appease the borrow checker. This
is deadly.

The entire premise of a startup is that *you have to iterate rapidly*. Very few
companies know what they should be building from day one. It's an iterative
process involving a feedback loop of talking to users and making changes to
reflect the feedback. The faster you can make that feedback loop, the higher
probability you have of success.

## Correctness

The evident hit in development velocity is redeemed to an extent by Rust's
emphasis on writing correct programs. "if it compiles it works' so to speak.
I've found this to be true for the most part while building with Rust and it is
an absolute joy to work with for this reason.

Even if your program is not perfect, you understand the failure modes much
better. The set of unknown failure modes is reduced substantially as your
program breaks in exactly the way you expect it to. The lack of null pointers in
conjunction with the `Result` paradigm (vs say, exceptions) compels you to build
correct programs where edge cases are well understood and are handled explicitly
by you (or `unimplemented!` but no one is perfect).

If you've reached product market fit - correctness may counteract the
development velocity hit. When you know what you're building you need to iterate
less. Your dev team is also going to be spending less time dealing with bugs as
you've already dealt with that while trying to appease the compiler.

If it compiles it works - and this is an invaluable asset when you're
aggressively shipping code.

## Talent

Getting great talent is unbelievably important for an early stage startup. The
fact that the absolute number of competent and experienced Rust developers is so
small initially seems detrimental to getting great people. This is exacerbated
by Rust's steep learning curve as you need to hire someone with experience, or
it's going to take months for them to become productive. However, this is not
the full picture.

In our experience the competence of your average Rust developer is much higher
than more conventional programming languages. Something spoke to these
individuals when they picked up Rust, and it's hard to put your finger on it but
it's that same quality that makes a great engineer. It's also been a pleasant
surprise to find out that really good engineers will seek you out as an
employer *because you use Rust*. They don't want to work in *script or Java or
C++. They want to work with Rust because it's great.

## Open Source

At Synth, we've chosen to adopt an open-core business model. The idea behind an
open-core business is you develop and open source product with a permissive
license which solves a real *technical* problem. You work on building a user
base, a community and a great product all out in the open. You then structure
your business model around solving the corresponding *organisational* problem -
and that's how you make money.

We've been really lucky to have a really active set of contributors - giving
ideas, reporting bugs and contributing (at times very significant) code. It is
hard to know for sure, but we have a strong hunch that a lot of the contributors
are active because they have an interest in Rust projects specifically. A lot of
our contributors are also interested in learning Rust - not necessarily being
veterans of the language. This has worked out great as the more experienced
members of our core team mentor and review code of young rustaceans, building a
symbiotic positive feedback loop.

Thank you to all our contributors - you know who you are and you guys are
amazing.

## Libraries

Rust has an ecosystem of incredibly high quality libraries. The Rust core team
has led by example and focused on a high quality and tight standard
library. The result of a highly focused standard library is (unfortunately) a
lack of canonical libraries for doing things outside the standard library. So
you want a webserver, pick from one of the 100s available. You want a crate (
Rust lingo for library) for working with JWT tokens? Here's 9, pick one. I mean,
even something as fundamental as an asynchronous runtime is split
between `tokio` and `async-std` and others. As a young rustacean this can 
be overwhelming.

What ends up happening over time is certain libraries become implicitly
canonical as they receive overwhelming support and start becoming serious
dependencies differentiating from their alternatives. Also in a project 
update from  RustConf 2021 it [was mentioned](https://youtu.be/ylOpCXI2EMM?t=1048) that the idea of having 'recommended crates' may be visited in the 
future. 

The lack of canonical non-standard libraries is an issue when you're getting
started - but over time this diminishes as you get a better understanding of the
ecosystem. What *has* been constantly detrimental to our development velocity
has been the lack of *client* libraries for Rust. We've had to write a
bunch of different integrations ourselves, but they're often clunky as we 
don't  have the time to invest in making them really high quality. For 
example most of Google's products have at best an unofficial code-generated 
crate maintained by the community, and at worst absolutely nothing. You 
need to write it from scratch.

## Should you build your startup with Rust?

Well it depends. Assuming you're building a product in the right domain for
Rust (say a CLI as opposed to a social media site), even then the answer is not
clear-cut. If you don't have close to 100% conviction that you know what you're
building, I would be inclined to say no. Development velocity and being able to
make rapid iterations is so important for an early stage startup that it
outweighs a lot of the benefits that Rust brings to the table.

If your company is later stage, and you now understand exactly what you should
be building (assuming this is every the case) then I would say yes. The 
'correctness' of Rust programs and the propensity of Rust to attract great
engineers can help in building a great engineering culture and a great company. 
