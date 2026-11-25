# Nothing checks that only the daemon links the keyring

Labels: area:daemon area:ci trigger:step

Invariants 1 and 3 are held by dependency edges that `arch-check.sh` refuses to see broken. Invariant 5's are tests alone: `grep keyring tools/arch-check.sh` finds nothing. One line in the existing edge check would make it structural.
