`roadmap/briefs/next-b3.md` at `4199160c`: a builder could start, but the done-when cannot be satisfied literally without unrelated edits.

- **Done when, second bullet — wording.** The declaration check confuses an item with every function sharing its name. `git grep -n -w 'query_words'` finds both the search function the pages use (`search_cmd.rs:1182`) and the saved renderer’s wrapper (`saved_cmd.rs:171`); `scope_words` has the same collision. Those saved renderers explicitly stay. Smallest change: check removal of each original declaration by its module and name, allowing unrelated declarations with the same name.

- **Find first, first command; Done when, first bullet — wording.** The discovery command matches ordinary words and method calls, making them completion failures despite their having no command-module dependency. Running it returns `assets/count.js:5` (“run”), `assets/page.css:77` (“not run”), and `web/mod.rs:720` (`next.run(request)`). It also misses `crate::auth_cmd::open_browser` at `web/mod.rs:628`, although decision 4 names the opener among the retained items. Smallest change: search module paths and imports, include `auth_cmd`, and apply the completion criterion to those dependencies rather than every matching text line.

What I did not check: compilation, tests, the gate, `cargo tree`, current CI, or running processes; this review used committed source and read-only searches only.

Form or prompt obstacles: none beyond the two checks identified above.
