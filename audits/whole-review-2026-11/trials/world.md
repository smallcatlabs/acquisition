# Trial C: the `world` document, sized

Thesis 3 says one document can say what a world holds and the words
that name each part. This is that document for the copy of the owner's
store, made by a script over the store files (read-only), to size it.
No product code was written.

```
_vagabond#6960  10 items  not logged in  --account '_vagabond#6960'
    pc · Standard  10 items  fetched 87d–87d ago  --realm pc --league Standard
GERWARIC#7694  44188 items  logged in  --account 'GERWARIC#7694'
    pc · Standard  43433 items  fetched 8d–5d ago  --realm pc --league Standard
    pc · Covens  604 items  fetched 8d–7d ago  --realm pc --league Covens
    poe2 · Standard  98 items  fetched 84d–84d ago  --realm poe2 --league Standard
    pc · Allflame  53 items  fetched 78d–78d ago  --realm pc --league Allflame
_exiled#6914  0 items  not logged in  --account '_exiled#6914'
```

590 bytes as text, 973 as compact JSON. The query behind each line is
`select realm, league, count(*), min(last_seen), max(last_seen) from
items where removed_at is null group by 1,2` on each account's facts
file, joined to `accounts.json`.

What it replaces as a first call: `acq accounts`, `acq store status`,
`acq leagues --stored` and the head of `acq tabs`, which the
first-answer investigator needed five calls and 4,917 bytes to
assemble, with one refusal on the way.

Not sized: what the document says of never-fetched places and of a
league the policy names but the store does not hold. Both are counts
the tool already computes.
