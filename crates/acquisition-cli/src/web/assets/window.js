// The table's window (step 6.4b, from step 6.3's measured rig): a table
// is a segment of rows of one height, with its route and its count, as
// the server rendered it, the first block of rows among them. The
// script sizes the segment, takes the server's block as one it holds,
// and on each scroll asks the blocks of rows near the viewport by offset
// and count, those in view first and the ones around them once those
// have come, places each at its offset, and drops the blocks gone far
// from it, the server's among them. The rows and their order are the
// server's; the script keeps the blocks it shows and the asks in flight,
// nothing a reload must keep. A moved basis (409) loads the page again.
(function () {
  const table = document.querySelector('.window[data-row-height]');
  if (!table) return;
  const h = Number(table.dataset.rowHeight);
  const BLOCK = Number(table.dataset.blockRows); // rows a block asks
  table.style.setProperty('--row', h + 'px');
  table.setAttribute('data-windowed', '');
  const segments = Array.from(table.querySelectorAll('.seg'), (el) => {
    const count = Number(el.dataset.count);
    el.style.height = count * h + 'px';
    const seg = { el, count, base: Number(el.dataset.base), route: el.dataset.route, blocks: new Map() };
    // the blocks the server rendered: held, done, and dropped as any other
    for (const block of el.querySelectorAll('.block[data-block]')) {
      const b = Number(block.dataset.block);
      block.style.top = b * BLOCK * h + 'px';
      seg.blocks.set(b, { el: block, abort: new AbortController(), done: true });
    }
    return seg;
  });

  const ask = (seg, b) => {
    const el = document.createElement('div');
    el.className = 'block';
    el.style.top = b * BLOCK * h + 'px';
    const abort = new AbortController();
    const block = { el, abort, done: false };
    seg.blocks.set(b, block);
    seg.el.appendChild(el);
    const offset = seg.base + b * BLOCK;
    const count = Math.min(BLOCK, seg.count - b * BLOCK);
    fetch(seg.route + '&offset=' + offset + '&count=' + count, { signal: abort.signal })
      .then(async (res) => {
        if (res.status === 409) {
          location.reload();
          return;
        }
        const text = await res.text();
        if (abort.signal.aborted) return;
        el.innerHTML = text;
        block.done = true;
        queue();
      })
      .catch(() => {
        // aborted: the block was dropped
      });
  };

  // the blocks of a segment's rows from lo to hi
  const span = (lo, hi) => {
    const first = Math.floor(lo / BLOCK);
    return [first, lo < hi ? Math.floor((hi - 1) / BLOCK) : first - 1];
  };
  // the blocks a segment wants: those in view, then those within one
  // viewport of its edges once no block in view is still asked
  const fill = () => {
    queued = false;
    const vh = window.innerHeight;
    const around = [];
    let waiting = false;
    for (const seg of segments) {
      const top = seg.el.getBoundingClientRect().top;
      const rows = (from, to) => [Math.max(0, Math.floor(from / h)), Math.min(seg.count, Math.ceil(to / h))];
      const [first, last] = span(...rows(-top - vh, 2 * vh - top));
      const [shown, end] = span(...rows(-top, vh - top));
      for (const [b, block] of seg.blocks) {
        if (b < first || b > last) {
          block.abort.abort();
          block.el.remove();
          seg.blocks.delete(b);
        }
      }
      for (let b = first; b <= last; b++) {
        const inView = b >= shown && b <= end;
        if (!seg.blocks.has(b)) {
          if (inView) ask(seg, b);
          else around.push([seg, b]);
        }
        if (inView && !seg.blocks.get(b).done) waiting = true;
      }
    }
    if (!waiting) for (const [seg, b] of around) ask(seg, b);
  };
  let queued = false;
  const queue = () => {
    if (queued) return;
    queued = true;
    requestAnimationFrame(fill);
  };
  window.addEventListener('scroll', queue, { passive: true });
  window.addEventListener('resize', queue);
  fill();
})();
