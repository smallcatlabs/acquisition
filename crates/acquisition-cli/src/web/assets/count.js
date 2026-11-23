// The count (`acq web`): beside each form that submits a search, how
// many items the form as it stands would match, asked of the server's
// `count` route as the form's fields change. It writes numbers alone,
// into the outputs the server rendered beside the form's button; every
// word is the server's, a refusal's its title. It keeps a timer, the ask
// in flight and the fields it last asked, nothing else, and never
// touches the answer's rows or the URL: Search is still how a count
// becomes the page's answer. A form's outputs are found among its
// elements, so a form whose controls stand in another's markup, owned by
// `form=`, has its count there too; and the page's input is heard on the
// document, each event counted for its target's form, or, for a target
// that has none (the stat pane's notice that an item list changed), for
// the form it stands in. So no form's count asks another's fields.
(function () {
  // each counted form's handler of a change
  const counted = new Map();
  document.querySelectorAll('form[data-count]').forEach((form) => {
    const elements = Array.from(form.elements);
    const matched = elements.find((el) => el.matches('output[data-count="matched"]'));
    const undecided = elements.find((el) => el.matches('output[data-count="undecided"]'));
    if (!matched || !undecided) return;
    const part = undecided.parentElement;
    let timer = null;
    let asking = null;
    // the fields the form sends, as the count asks them
    const fields = () => new URLSearchParams(new FormData(form)).toString();
    // the fields last asked: the page's own until the form changes
    let asked = fields();
    const dash = () => {
      matched.value = '–';
      matched.title = '';
      part.hidden = true;
    };
    const ask = async () => {
      if (asking) asking.abort();
      const mine = new AbortController();
      asking = mine;
      dash();
      try {
        const res = await fetch('count?' + asked, {
          headers: { Accept: 'application/json' },
          signal: mine.signal,
        });
        const doc = await res.json();
        if (mine.signal.aborted) return;
        if (!res.ok) {
          matched.title = doc.error || '';
          return;
        }
        matched.value = String(doc.total.matched);
        undecided.value = String(doc.total.undecided.count);
        part.hidden = doc.total.undecided.count === 0;
      } catch (e) {
        // aborted by a newer ask, or failed: the dash stays
      }
    };
    const changed = () => {
      // an event that leaves the fields as last asked asks nothing: a
      // control no form sends (a finder's box) among them
      const now = fields();
      if (now === asked) return;
      asked = now;
      // a change makes the number shown and the ask in flight the old
      // form's: both go at once, and the new form is asked after the pause
      if (asking) asking.abort();
      dash();
      clearTimeout(timer);
      timer = setTimeout(ask, 300);
    };
    counted.set(form, changed);
  });
  const heard = (event) => {
    const target = event.target;
    const form = target.form || (target.closest ? target.closest('form') : null);
    const changed = form && counted.get(form);
    if (changed) changed();
  };
  document.addEventListener('input', heard);
  document.addEventListener('change', heard);
})();
