# Opening one item takes 187 ms: `show` reads every row to find one id

Labels: area:search area:frontends trigger:gui

`item/<id>` took 186 to 189 ms for 3.9 KB on the owner's 44,090 items (`trials/gui-in-place.md`); `acq show` 0.26 s. A row opened from the held corpus costs less than the whole page it sits in. Unfiled since the performance read of 2026-10-19.
