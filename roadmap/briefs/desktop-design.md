# desktop-design — The desktop GUI: what it is, and the road to parity

A Fable design, 2026-11-27, written after the owner sat at the
prototype (`audits/desktop-2026-11/spike/README.md`, the spike's
record, and `roadmap/briefs/desktop-spike.md`). Nothing here is built
or ruled. The first section is for the owner; the rest is for the
session that writes the build briefs after his word. The prototype
is throwaway and stays out of the product; what it proved is kept.

## To the owner, in plain words

**What the prototype settled.** A window drawn by a browser engine
over the search the tool already has is fast enough. On your 43,433
Standard items, at your sitting, a scroll took 2 ms, opening a tab 50
ms, opening an item 56 ms, a keystroke 80 ms; the C++ app waits 350
ms before it starts to refilter. The browser's part of an act was a
few milliseconds in the measured runs and at most one frame of your
display at the sitting. What is slow is the search behind it, and
that is the same under any toolkit. So "web or native" is closed by
the evidence: in the measured runs a native toolkit would speed up
the part of a keystroke that takes 3 ms and leave the part that
takes 120.

**The framing that makes this small.** The desktop GUI is not a new
engine and not a new kind of program. It is the prototype's shape
made a product: one Rust program that holds your items in memory and
answers a page that only draws. Almost all of the road to parity is
then two kinds of work the project already knows how to do: making
the search answer what a tree needs (which also speeds the command
line and the agent's door), and adding features one at a time.

**Three things are yours to rule, in this order.** Each comes to you
as one question when you are ready for it; this is the map.

1. *What it is.* A fourth frontend beside the command line, the
   agent's door and `acq web`: its own program, the prototype's
   shape. `acq web` stays as it is.
2. *What the window is.* A browser's app window first, as the
   prototype was; a packaged app with its own icon later, as a
   wrapper around the same program.
3. *The order.* The search's six limits first, then a read-only
   window to parity with the C++ app's browsing, then refreshing
   from the window, then pricing, then the shop.

## 1. What the spike showed, and did not

Held by the record: the bar, the judged runs, the sitting's log and
the owner's words. In short:

- In the headless runs the browser's share of an act was 1 to 8 ms
  and the transport's about 1 ms; at the sitting the browser's was
  at most a display frame. The rest was the engine's.
- The bar missed on two lines by one act each in headless runs: a
  keystroke that waits behind an answer the engine cannot stop, and
  one late frame that only a display can judge.
- The page stayed presentation: every row, count and order came from
  the search crate, and a check holds the rows drawn to the rows
  counted (C46, semantics in Rust).
- The owner: "shockingly good", "lacking features and polish and the
  UI elements don't all behavior correctly", "line of sight to
  feature parity and beyond with the c++ app".

Not shown: Windows, where most of the C++ app's users are; a store
that changes while the window is open; any write; item art; a
second browser engine; frames counted on a real display; a person
typing the search's language at speed (the sitting had four
keystrokes, all in number fields).

## 2. What it is

**A fourth frontend, its own crate and binary.** It links what a
frontend may (C1, the crate edges; C12, a frontend's two surfaces):
the verbs, the search, the store's read API and the client. It
serves one page over loopback and holds the corpus in memory for
its lifetime. The page draws documents and evaluates nothing (C16,
C46).

Options weighed:

| | Buys | Costs |
| --- | --- | --- |
| **Its own crate and binary** (recommended) | The CLI crate stops growing; the page-per-act pages and the one-window program never share a router; C191 holds by construction | One more release artifact; C1's table gains a row |
| A verb of the CLI, `acq desktop` | No new artifact | The CLI crate is already in 46% of product commits and holds `acq web`; two page models in one crate is the warp C191 forbids |
| Reshape `acq web` | One GUI | Ruled out by the owner (C191) |
| A native toolkit | Nothing the measurements can find | A second rendering of every answer, outside Rust's documents; a toolkit's own supply chain |

**What it keeps of `acq web`'s rulings.** The listener's guard
(C138: loopback, a per-launch token, the Host check, POST for
writes) and the write door (C168) apply unchanged. C16's sentence
that a page holds no state the server cannot rebuild from its URL
held in the spike for everything but the scroll position, and is
kept. What changes is one mechanism, not a property: a new search
replaces the page's contents in place instead of loading a page.

**What it does not share with `acq web`.** Markup, routes and
goldens. It shares the documents and the words, which is why B3
(the shared words out of the CLI's binary) comes before it.

## 3. What the window is

| | Buys | Costs |
| --- | --- | --- |
| **A browser's app window first** (recommended) | No new dependency; no packaging or signing; the engine the spike measured | Needs a Chromium-family browser; no icon of its own; the browser may contact its maker at start |
| A packaged shell from the start (Tauri, as C16 names) | A real app: an icon, one download | The system's own web engine on each platform, which the spike did not measure; a new dependency tree; a signing and notarising story on two platforms before any feature |

The second is a wrapper around the first and loses nothing by
waiting (C16's own reason: a webview can point at the same server
later). It is taken up when the layout has settled, with its own
measurement on each platform's engine.

## 4. The road to parity

**First, the engine.** The six limits the spike found, each a step
in the search or the store, each paying on every surface:

1. A place's items without a scan of the corpus.
2. Counts by location in one answer.
3. A places document that does not re-read the tab listing on each
   query.
4. An answer that can be abandoned when a newer one is asked.
5. Row building, about 14 µs a row today.
6. The currency check, 10 ms on every act.

And the eight rules the rig wrote in its own Rust (the record, "The
rules the rig wrote in Rust"): each becomes the engine's, shared and
tested once, or is ruled out. The form's fold is the one with a
choice in it: `acq web` folds a form shaped like the trade site's,
and the desktop's is shaped like the C++ app's. One vocabulary of
fields with two layouts is the position; a second fold is a second
semantics.

**Then, in order, each its own design where one is named:**

| Slice | What a C++ user gets | Needs |
| --- | --- | --- |
| A. The read-only window | The whole search form, search tabs, every column, sort, keys, item art, the tooltip as the game draws it | The engine steps; a Windows sitting; item art under C140 |
| B. Refresh from the window | "Update" with progress, and a list that changes under you | A design: the live feed that 4-full's parts 2 and 3 were to give `acq web` is designed once, here |
| C. Pricing | Buyouts set from the item panel | The write door (C168), designed for the panel |
| D. The shop | The forum pages | `shop render` as it is (C120: a person pastes) |

Slice A is where "feels like the C++ app" is won or lost for a
long-time user, and it sends nothing to GGG.

**How it is tested.** By documents and by acts, never by a page's
markup: the server's answers as goldens of documents, the spike's
check (the rows drawn are the rows counted) as a test, and the
browser-driven stage the gate has (C187) for what only a page can
show. `acq web`'s 795 KB of markup goldens is the cost this avoids.

## 5. What not to do

- Do not carry the rig's code into the product. Its patches are the
  record; the product crate is built from briefs, reviewed as any
  build.
- Do not design slices B to D now. Each waits for the slice before
  it to be sat at.
- Do not retire `acq web`: not until the desktop window does what
  its pages do, and then at the owner's word.
- Do not add a framework, Node or a bundler: the rig's one
  hand-written page was enough at this size (the frontends'
  "Not doing" line stands).

## 6. Drafts for the owner

Each a registry line to be written in his words once he rules.

- C191 gains its second half: the desktop GUI is a fourth frontend,
  its own crate and binary, a Rust program holding the corpus and one
  page that draws it.
- C1's edges gain the frontend's row.
- C16 says which of its sentences bind the desktop page as they
  bind `acq web`'s, and that in-place replacement is a mechanism.
- The road: 4-full's parts 2 and 3 are designed for the desktop
  window, not for `acq web`.

## 7. Not checked

- Windows, and WebView2 or WebKit as the engine.
- Whether the six engine steps can each land without changing an
  answer's document; none is designed here.
- What "the UI elements don't all behave correctly" named: the
  owner's list, when he gives it, is slice A's first input.
- Memory: the corpus held for a session's life was not measured.
