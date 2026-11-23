// The stat pane's lists and the finders (`acq web`): the rows and groups
// the server rendered, copied and removed. One click listener on the pane
// serves two verbs: an add control (`data-add`) copies the `<template>` it
// holds, or the one its attribute names, into the item list (`data-items`)
// of its nearest list (`data-list`); a remove control (`data-remove`)
// removes its nearest item (`data-item`). The item list that changed then
// announces it with one bubbling `change`, and after an add the copy's
// element marked `data-focus` takes the focus. A finder (`data-finder`) is
// a box no form sends and a listbox, heard wherever it stands, its
// listeners on the document: it asks the route its attribute names, the
// server's, with the words typed appended, and puts the answer's options
// in its list, the first marked (`aria-selected`); the list is busy
// (`aria-busy`) from an ask until it is answered, fails or is retired, so
// no answer is on its way to a list not busy; arrows move the mark,
// Enter clicks the marked option and never submits, and Escape empties it.
// What an option's click does is the option's own. The script names no
// field and writes no word and no value a form sends: every row, group,
// option and word is the server's.
(function () {
  const panes = Array.from(document.querySelectorAll('[data-list]'))
    .filter((el) => !el.parentElement.closest('[data-list]'));
  // a finder's ask in flight, aborted when the finder is emptied or asks
  // again; its list is busy while it holds one
  const asking = new WeakMap();

  function retire(finder) {
    const ask = asking.get(finder);
    if (ask) ask.abort();
    asking.delete(finder);
    finder.querySelector('[role=listbox]').removeAttribute('aria-busy');
  }

  // empties a finder, its box and its list, and retires its asks in
  // flight, so no late reply fills it again
  function empty(finder) {
    retire(finder);
    finder.querySelector('input').value = '';
    finder.querySelector('[role=listbox]').replaceChildren();
  }

  function marked(finder) {
    return finder.querySelector('[role=option][aria-selected=true]');
  }

  function mark(finder, option) {
    const old = marked(finder);
    if (old) old.removeAttribute('aria-selected');
    if (option) {
      option.setAttribute('aria-selected', 'true');
      option.scrollIntoView({ block: 'nearest' });
    }
  }

  async function ask(finder) {
    retire(finder);
    const list = finder.querySelector('[role=listbox]');
    const words = finder.querySelector('input').value.trim();
    if (words.length < 2) { list.replaceChildren(); return; }
    const mine = new AbortController();
    asking.set(finder, mine);
    list.setAttribute('aria-busy', 'true');
    try {
      const res = await fetch(finder.dataset.finder + encodeURIComponent(words), { signal: mine.signal });
      const text = await res.text();
      if (mine.signal.aborted || !res.ok) return;
      list.innerHTML = text;
      mark(finder, list.querySelector('[role=option]'));
    } catch (e) {
      // aborted by a newer ask or an emptying, or failed: the list stands
    } finally {
      // answered or failed; an ask a newer one or an emptying retired has
      // left the list to them
      if (asking.get(finder) === mine) retire(finder);
    }
  }

  function add(control) {
    const template = control.querySelector(':scope > template')
      || document.getElementById(control.dataset.add);
    const list = control.closest('[data-list]');
    const items = list && list.querySelector(':scope > [data-items]');
    if (!template || !items) return;
    const copy = template.content.firstElementChild.cloneNode(true);
    items.appendChild(copy);
    const menu = control.closest('details');
    if (menu && list.contains(menu)) menu.open = false;
    items.dispatchEvent(new Event('change', { bubbles: true }));
    const focus = copy.matches('[data-focus]') ? copy : copy.querySelector('[data-focus]');
    if (focus) focus.focus();
  }

  function remove(control) {
    const item = control.closest('[data-item]');
    if (!item) return;
    const items = item.parentElement;
    item.remove();
    items.dispatchEvent(new Event('change', { bubbles: true }));
  }

  for (const pane of panes) {
    pane.addEventListener('click', (event) => {
      const control = event.target.closest('[data-add], [data-remove]');
      if (!control || !pane.contains(control)) return;
      if (control.hasAttribute('data-add')) add(control); else remove(control);
      const finder = control.closest('[data-finder]');
      if (finder) empty(finder);
    });
  }
  // a finder is heard wherever it stands: in the stat pane, in a form
  // that carries the page's view, beside a place page's heads
  document.addEventListener('input', (event) => {
    const finder = event.target.closest && event.target.closest('[data-finder]');
    if (finder) ask(finder);
  });
  document.addEventListener('keydown', (event) => {
    const finder = event.target.closest && event.target.closest('[data-finder]');
    if (!finder) return;
    const options = Array.from(finder.querySelectorAll('[role=option]'));
    const at = options.indexOf(marked(finder));
    if (event.key === 'ArrowDown' || event.key === 'ArrowUp') {
      event.preventDefault();
      if (options.length === 0) return;
      const step = event.key === 'ArrowDown' ? 1 : -1;
      mark(finder, options[(at + step + options.length) % options.length]);
    } else if (event.key === 'Enter') {
      // Enter in a finder picks; it never submits the form
      event.preventDefault();
      if (at >= 0) options[at].click();
    } else if (event.key === 'Escape') {
      empty(finder);
    }
  });
})();
