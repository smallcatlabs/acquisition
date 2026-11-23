`roadmap/briefs/4-full-1-form-landing.md` at `800c594c`: a builder needs clarification before starting.

- **Changes what gets built — Done when, refusal census.** Requiring every refusal to offer a request or name a terminal command excludes ordinary validation errors that the restored form already remedies. An empty saved name becomes `Arguments` in `saved.rs:431`; `refusal::document` supplies no offer, and its words name no command. The builder must first guess whether the restored form qualifies. Smallest change: explicitly accept a restored editable form and its way back as a remedy, without requiring a new offer or command.

- **Changes what gets built — Decisions 2–3 / Not in scope.** The place-or-step classification leaves `GET saved/open` unclassified despite requiring step links to carry `from`. The router mounts it, `saved_open` redirects to search, and its failures render a refusal; the saved list and `Paged::request` both link to it. Smallest change: explicitly classify this opening flow and specify whether it carries `from`; if excluded, name that exclusion under “Not in scope.”

- **Wording — Done when, `from` validation.** “Writes none into its own forms” contradicts Decision 3’s requirement that leaving forms carry the current page’s `From`. For `search?text=boots&from=tabs…`, the save form must carry `search?text=boots`, while omitting the incoming return destination. Smallest change: say that forms never copy the incoming `from`, and that leaving forms carry the page’s own route and fields instead.

- **Wording — Find first, answer and test discovery.** The commands miss callers affected by the prescribed changes. Running them omitted the direct `page::refused` calls at `web/mod.rs:1449` and `:2349`, and the `place_form` helper at `tests/web.rs:4123`; that helper’s callers currently expect the plan document immediately, whereas Decision 4 inserts a redirect. Smallest change: add searches for `page::refused(`, `place_form(` and its definition, and require callers to follow the new redirect before asserting on the plan.

- **Wording — Done when, census provenance.** Defining the test’s rows as “the report’s” makes that correspondence unavailable to a reviewer restricted to the commit. Smallest change: require the census mapping and exclusion reasons in the committed test or commit message.

What I did not check: builds, tests, browser behavior, goldens regeneration or the gate, as instructed. Issue comments and external CI evidence were not verified; GitHub reads failed because `api.github.com` was unreachable.

Form impediment: “last comment” and “last two” identify mutable inputs; permanent comment links would make the brief reproducible.

Prompt impediment: none.
