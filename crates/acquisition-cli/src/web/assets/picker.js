// The stat picker (`acq web`): asks the server's stat vocabulary for the
// words typed and writes a chosen stat's id into the first empty stat
// row. It evaluates nothing: which stats match and each one's fate are
// the server's answer, listed as given.
(function () {
  const find = document.getElementById('stat-find');
  const list = document.getElementById('stat-list');
  if (!find || !list) return;
  let asked = 0;
  find.addEventListener('input', async () => {
    const mine = ++asked;
    const words = find.value.trim();
    if (words.length < 2) { list.replaceChildren(); return; }
    const res = await fetch('stats?q=' + encodeURIComponent(words), { headers: { Accept: 'application/json' } });
    if (mine !== asked || !res.ok) return;
    const doc = await res.json();
    const items = doc.entries.map(entry);
    if (doc.left_out > 0) {
      const more = document.createElement('li');
      more.textContent = doc.left_out + ' more match: add a word';
      items.push(more);
    }
    list.replaceChildren(...items);
  });
  function entry(e) {
    const li = document.createElement('li');
    const pick = document.createElement('button');
    pick.type = 'button';
    pick.textContent = e.texts.join(' / ');
    pick.addEventListener('click', () => {
      const slots = Array.from(document.querySelectorAll('input.stat-id'));
      const slot = slots.find((s) => !s.value) || slots[0];
      if (!slot) return;
      slot.value = e.id;
      const said = document.getElementById(slot.id + '-text');
      if (said) said.textContent = e.texts.join(' / ');
    });
    const fate = document.createElement('span');
    fate.className = 'fate';
    fate.textContent = e.group + ' · alone ' + e.alone + ' · with a bound ' + e.bounded;
    li.append(pick, ' ', fate);
    return li;
  }
})();
