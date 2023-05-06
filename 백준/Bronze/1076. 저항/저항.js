const list = [
  ['black', 0],
  ['brown', 1],
  ['red', 2],
  ['orange', 3],
  ['yellow', 4],
  ['green', 5],
  ['blue', 6],
  ['violet', 7],
  ['grey', 8],
  ['white', 9],
];
const [a, b, c] = `${require('fs').readFileSync(0)}`.trim().split('\n');
console.log(
  (list.findIndex(x => x[0] === a.trim()) * 10 +
    list.findIndex(x => x[0] === b.trim())) *
    10 ** list.find(x => x[0] === c.trim())[1],
);