# The GUI's direction

The plan's D1. Investigator's report: `reports/investigators/gui.md`. Trial: `trials/gui-in-place.md`. Portfolio: proposal 2.

## What was found

The gap between `acq web` and the C++ app is in the client, not the server. On the owner's 44,090 items the server answers a filter, a window of rows or a sort of every row in 30 to 120 ms. What a person meets is that each act (open a row, sort, pick a value, open a tab) is a 303 and a whole page of 87 to 150 KB, after which focus is gone and an anchor restores the scroll. The C++ app is one process with a model in memory: an edit restarts a 350 ms timer and the view refilters in place.

`acq web` is 16 routes; each also serves its verb's document as JSON. The durable part is the routes, their documents and the form's fold with its exact inverse. The markup is the disposable part, and it is what 795 KB of goldens and most of an 8,339-line test file hold.

## Checked by the orchestrator

- **C191 is the owner's own ruling**, in his words of 2026-11-19 (`f7e1e10d`): "acq web is helping us bootstrap acq's development. it was never intended to be a high performance desktop app, and it would be wrong to warp it into such." The investigator proposes striking it. That is his to weigh against his own standing direction that the C++ layout and its speed are the reference.
- **The trial.** In a real browser on his store, replacing the answer region in place took 33 to 167 ms to the next paint over nine queries; the browser's share was 3 to 4 ms. One item took 187 ms. Run while another agent compiled, so upper bounds.
- C16's script scan refuses any served script containing the word `history` (`web/mod.rs:224-236`), which forbids `replaceState`, the API that keeps the URL the whole state when a page updates itself.
- Not re-run alone: the investigator's per-route `curl` timings. The trial's numbers agree with them within noise.

## Where this parts from the plan

- The web scout said URL-as-state and a full re-render cannot give C++ interactivity. The server can; the navigation per act cannot.
- The advisor's T3 calls the form a second query language of near-equal weight. It is a one-way input spelling: every answer, saved search and route carries the language's text, so the language is already the model. What is missing is the partial inverse that fills the form from a text shaped like a site search.
- The advisor's T11 fears the route JSON becomes a third surface. Not while the client ships in the same binary as its routes.

## Position

C16 already rules that any GUI is web rendering over loopback. So the separate GUI that C191 reserves would be another client on the same kind of server, and the only thing that separates it from `acq web` is how the client behaves. The trial says that behaviour is cheap: in-page acts become swaps of a region fetched from the same URL, and the landing machinery those acts needed goes (`toggle_item`, `at`, `cut`, `picked`'s 303). That removes more than it adds.

What would change this: the owner types in a prototype on his store and it does not feel right; or "C++ performance" means a million items, which is a question about the corpus's memory and no GUI choice answers it.

The decision is his, and it should be made before parts 2 and 3 are designed, since they design a live feed for a surface his ruling calls temporary.

## Route

Proposal 2, one yes-or-no question. Issues drafted: `show-scans-the-store-for-one-id`, `currency-check-reads-the-whole-header`, `every-answer-page-resends-the-form`, `a-golden-with-no-facts`, `readme-says-there-is-no-ui`.

## After the skeptic

Narrowed (`dissent/theses-1-5.md`, thesis 2).

- The trial was run while another agent compiled, which C185 forbids for a measurement. The numbers are evidence that the idea is worth trying, not a result.
- Replacing the history entry on a new search breaks Back: search A, search B, Back should return to A. A new search is a navigation or a pushed entry; only an act inside one answer replaces.
- `cut` (`web/mod.rs:2603`) is not landing machinery. It rebuilds the window's segments so that an opened row outside the first block is still there on reload. It stays.
- The page's basis sits in the head (`assets/page.html:7`), outside the region the trial swapped. An in-place update has to carry its basis or C16 is broken.

What stands: the server is not the bottleneck; each act is a page load; the road is about to design a live feed for a surface ruled temporary. The question for the owner is smaller than "strike C191": whether he wants to sit at a prototype on his own store before parts 2 and 3 are designed.
