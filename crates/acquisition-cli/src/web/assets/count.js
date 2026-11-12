// The count (`acq web`): beside each form that submits a search, how
// many items the form as it stands would match, asked of the server's
// `count` route as the form changes. It writes numbers alone, into the
// outputs the server rendered beside the form's button; every word is
// the server's, a refusal's its title. It keeps a timer and the ask in
// flight, nothing else, and never touches the answer's rows or the URL:
// Search is still how a count becomes the page's answer.
(function () {
  document.querySelectorAll('form[data-count]').forEach((form) => {
    const matched = form.querySelector('output[data-count="matched"]');
    const undecided = form.querySelector('output[data-count="undecided"]');
    if (!matched || !undecided) return;
    const part = undecided.parentElement;
    let timer = null;
    let asking = null;
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
        const fields = new URLSearchParams(new FormData(form));
        const res = await fetch('count?' + fields, {
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
    const changed = (event) => {
      // a control the form never sends (the picker's own box) asks nothing
      if (!event.target.name) return;
      clearTimeout(timer);
      timer = setTimeout(ask, 300);
    };
    form.addEventListener('input', changed);
    form.addEventListener('change', changed);
  });
})();
