// A site search's link back to the trade site in a league of its own (a
// link may carry a league a person names for it alone): the server wrote
// the link once per league the form names, each the value of an option
// of the select beside it, and the select swaps the chosen one into the
// link. The script composes no link and asks
// nothing; the search here does not change.
document.addEventListener('change', (event) => {
  const select = event.target.closest('select[data-link-league]');
  if (!select) return;
  const link = select.parentElement.querySelector('a[data-link]');
  if (link) link.href = select.value;
});
