# A waiter rekeyed while it waits drops the permit it is granted and queues again

Labels: area:network trigger:step

`mapping_is_current` (`choke.rs:491`) compares the key it asked under with the key now, so a waiter the gate rekeyed in place is granted, found stale, released and sent to the back. Harmless to the limit; loses first-in order.
