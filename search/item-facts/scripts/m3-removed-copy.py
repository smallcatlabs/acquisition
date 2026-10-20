#!/usr/bin/env python3
"""The removed-heavy copy M3 also asks over (roadmap 0a): the M3 copy with
removed items added and its live set unchanged, so a store's age is
measured — the M3 copy itself holds 6 removed items.

    python3 -I search/item-facts/scripts/m3-removed-copy.py \
        search/item-facts/raw/spike-GERWARIC_7694-2026-09-13.db \
        search/item-facts/raw/removed-heavy.db

Deterministic: a `.backup` of the source, then COPIES removed rows per
live item, in id order — each a copy of the live row under the id
sha256(`<id>/removed/<k>`) in hex, shaped as GGG's ids are so a query
can name it, at the live item's own realm, league and location
(so the removed spread over the live locations and leagues exactly as
the live do), its socket parent renamed the same way, `removed_at` its
`last_seen`. Nothing else changes. Beside the copy, `<copy>.manifest.json`
records the source's sha256, the counts before and after, and these
parameters; a run that differs from the manifest is a different copy.
Never in the gate: the input is raw/.
"""
import hashlib, json, os, sqlite3, subprocess, sys

COPIES = 4


def sha256(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for block in iter(lambda: f.read(1 << 20), b""):
            h.update(block)
    return h.hexdigest()


def counts(db):
    live, removed = db.execute(
        "SELECT sum(removed_at IS NULL), sum(removed_at IS NOT NULL) FROM items"
    ).fetchone()
    return {"live": live, "removed": removed}


def main():
    source, out = sys.argv[1], sys.argv[2]
    if os.path.exists(out):
        os.remove(out)
    subprocess.run(["sqlite3", source, f".backup '{out}'"], check=True)
    db = sqlite3.connect(out)
    before = counts(db)
    columns = [r[1] for r in db.execute("PRAGMA table_info(items)")]
    rename = {"id", "socketed_in", "removed_at"}
    kept = [c for c in columns if c not in rename]
    db.create_function("removed_id", 2, lambda i, k: None if i is None else
                       hashlib.sha256(f"{i}/removed/{k}".encode()).hexdigest(), deterministic=True)
    for k in range(1, COPIES + 1):
        db.execute(
            f"""INSERT INTO items ({", ".join(["id", "socketed_in", "removed_at", *kept])})
                SELECT removed_id(id, ?1), removed_id(socketed_in, ?1), last_seen, {", ".join(kept)}
                  FROM items WHERE removed_at IS NULL ORDER BY id""",
            (k,),
        )
    db.commit()
    after = counts(db)
    leagues = db.execute(
        """SELECT realm, league, location_kind, count(*) FROM items
            WHERE removed_at IS NOT NULL GROUP BY 1, 2, 3 ORDER BY 1, 2, 3"""
    ).fetchall()
    db.close()
    manifest = {
        "source": os.path.basename(source),
        "source_sha256": sha256(source),
        "before": before,
        "after": after,
        "construction": {
            "copies_per_live_item": COPIES,
            "id": "sha256('<live id>/removed/<k>') in hex, k = 1..copies",
            "location": "the live item's own realm, league, location kind and id",
            "removed_at": "the live item's last_seen",
        },
        "removed_by_realm_league_kind": [list(r) for r in leagues],
    }
    with open(out + ".manifest.json", "w") as f:
        json.dump(manifest, f, indent=2)
        f.write("\n")
    print(json.dumps(manifest, indent=2))


main()
