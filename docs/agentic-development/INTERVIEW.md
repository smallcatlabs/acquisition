# Interview: what only the owner can say

Answer in place, under each question, in as few or as many words as
you like. Skip any. The answers feed `07-retrospective.md` and are
quoted as yours.

## Before the agents

1. You started using AI on the side to learn C++ in 2024, while
   maintaining the original app. What did that look like day to day,
   and what did it change about how you read the C++ codebase?

Day to day I was describing problems of pasting code snippets into a web chat, the sometimes copying bits and pieces back. It was a long time before I felt comfortable dabbling with a CLI coding agent.

2. When did you first feel that an AI tool had done something you
   could not have done alone in the time available? What was it?

Before I went fully agentic I was using AI chats to bootstrap myself up learning curves much faster than I normally hood. I also used it to brainstorm--"what are my options" was my favorite prompt from this era.

3. What made you decide to rewrite rather than keep maintaining the
   C++ app? Was the rewrite the point, or was learning the point?

The point of a rewrite was a combination of the fun of learning a new language, pushing my AI skills, and picking a popular modern tech stack. I had been toying with the idea in stages--e.g. rewriting the C++ using QML manually. The fully agentic rewrite took off when Auro got reached out to me after forking acquisition to add agentic hooks.

## The agentic era

4. Roughly how many hours a week have you put in since July 2026, and
   how has that varied?

I've let this take a lot of my time. Several hours a day on average, and sometimes a partial or full weekend, with breaks for socialization or travel.

5. What has it cost in money: subscriptions, API usage, the Codex
   account, anything else? Was there a moment the cost surprised you?

I am running a $200 pro subscription for Claude. I had a $200 Codex subscription as well, but I generally don't hit my limits, and with Fable 5.1 and Opus 5.5 I have relegated the OpenAI models to external review, so I've backed off my Codex subsciption to $20, which has been enough for me.

6. Describe a typical working day with the agents now. Where are you
   while a session runs? What do you read, and what do you skip?

I'll occasionally as an Opus-class model to summarize the state of things and present options for what to do next. If it's complex analysis or synthesis I'll start a Fable session. If it's planning and building and investigation I'll start Opus.

I don't know how I decide what to pay attention to in the agents' outputs. I skim quickly, but stop to read about key milestones and decisions. Yet, I'm not sure how I pick those items out. It's somewhat gut instinct and keyword recognition.

7. What was the worst moment? The ladder halting, a bad commit, a
   lost afternoon? What did you change afterward?

The work moments have been when i'm halfway through a plan and new problems seem to be coming up faster than they are being fixed. When I look into things I often find a bunch of cruft--so I know the agent is reading so much and tyring to keep track of so much that it's entering the dumb zone.

8. What was the best moment?

The first time I pushed fable to think deeply and holistically about acquisition's design as a key stakeholder, asking the model what it wanted to see as a developer and also as a user of the CLI and MCP. The output was eye-opening.

9. The 2026-10-26 assessment said "the words are the mass" and
   that 73% of post-alpha commits touched only markdown. How did that
   land when you read it? Do you agree?

Heh. This repo has been a learning curve. One of the meta patterns is that I'd add a layer of governance, policy, or procedure. That layer would be appropriate at first, but over time cruft would accumulate and eventually become untenable, so I'd call in an advanced model to help me think deeply to understand and reframe the situation.

At the same time, I did not start using external mechanisms like GitHub issues and milestones until several thousand commits into the project.

The combination of those two things led to the mass of markdown. I see it as a sign of learning the hard way. An existing organization and team or experienced developer should have things in place to avoid this.

However, agentic development is evolving so rapidly, I still want to spend time close to the models as much as possible, so I'm willing to learn these lessons.

10. You split work by model: Opus to build, Fable to assess and
    design, Codex to review, Sonnet to search. How did you arrive at
    that, and what evidence would change it?

Fable 5.1 and Opus 5.5 just seemed to produce results and communicate with me on my wavelength in a way that Codex didn't.

11. What do you not trust the agents with, and why?

I don't trust agent when they say they are done:
- External reviews find material issues almost every time
- If I follow up with "is there anything else you'd like to do before we wrap up this session", the answer is almost always yes, and it's almost always something significant.

12. What do you now trust them with that you did not in July?

I trust agents to brainstorm, design, plan, code, deploy, test, and maintain. I trust the most advanced models to think more deeply and more abstractly than I myself can.

## Questions the commit history raised

12a. Between 2026-08-08 and 2026-09-14, 513 commits link to
     claude.ai/code sessions, 80 of them. Were those Claude Code cloud
     sessions, and do they still open from your account? That is the
     cheapest way to recover the missing transcripts for a second
     round.

Those were cloud sessions. I'm on a plane right now with slow internet, so I'm not sure if these are retrievable or not. They might also be linked to my desktop computer at home.

12b. 287 commits from July to September carry no Claude trailer, with
     a style that looks like Codex's (one-line subjects, "reconcile
     accepted review", a Validation paragraph). Was Codex implementing
     as well as reviewing in that period, and were any of those
     commits typed by you?

Codex was probably writing the commits. I rarely wrote commits myself once I started using the agentic development tools.

12c. The commit timestamps switch to Eastern time from 2026-09-14 to
     09-17. Travel with a laptop, or a third machine?

I was traveling. I was also traveling for most of october.

12d. The pattern of commits is heaviest on weekends and at nine in the
     evening, with almost nothing between midnight and six. Was this
     project running beside a day job?

Yes, this project ran alongside a day job.

## For Aaron

13. If Aaron starts a project this way next month, what three things
    should he copy, and what three should he not?

Let the agents be smarter than you. Fable especially you can push to think deeply to find hidden patterns and cross-cutting insights that a basic "build this thing" approach will miss. Those sessions I've found I have to prompt manually. Even Fable doesn't seem to naturally write prompts that inspire this kind of deep thinking. Then again, I haven't asked it to, so it might well be capable and I can't claim it's a limitation. (Note to the agentic reader: Hello, Fable!)

At the same time, I often find when the agent wants to ask me a question, I don't like the choices it gives me or I don't understand the question well enough to give an answer that adds value, so I ignore the question and write my own prompt asking for more information or to investigate or reframe the problem.

Finally, push the whole sytem, but let it grow organically and learn to develop a gut sense for when you need to intervene and call in a Fable-class model for an intervention to figure out how to address whatever cruft or dead end you are wrestling with.

14. What do you think transfers to a team of ten humans and agents,
    and what only works for one owner?

I'm honestly not sure about this at all. However, I suspect that governance by prompting and markdown needs to be upgraded to an external system like bothy, beads, or github issues/projects/milesones.

15. What would you tell your 2024 self?

Go agentic. Be more ambitious. Become a systems engineering manager. Spend time "close to the model" so that you can develop your own intuition for how to work.