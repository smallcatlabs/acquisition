// The list filter (`acq web`'s refresh page): the boxes under a list's
// column heads hide each row whose cell does not hold a box's words, case
// aside, and a group (a folder, a tab's pages) while no row in it shows.
// It keeps nothing: the boxes are sent nowhere and a reload clears them.
document.querySelectorAll('details.list').forEach((list) => {
  const boxes = Array.from(list.querySelectorAll('input.col-filter'));
  const apply = () => {
    const wants = boxes
      .map((box) => [Number(box.dataset.col), box.value.trim().toLowerCase()])
      .filter(([, words]) => words !== '');
    list.querySelectorAll('tr.row').forEach((row) => {
      row.hidden = !wants.every(([col, words]) => {
        const cell = row.cells[col];
        return cell !== undefined && cell.textContent.toLowerCase().includes(words);
      });
    });
    list.querySelectorAll('.holds').forEach((group) => {
      const rows = Array.from(group.querySelectorAll('tr.row'));
      group.hidden = wants.length > 0 && !rows.some((row) => !row.hidden);
    });
  };
  boxes.forEach((box) => box.addEventListener('input', apply));
});
