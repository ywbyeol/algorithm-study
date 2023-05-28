const arr = `${require('fs').readFileSync(0)}`.trim().split('\n');
const map = new Map();
arr.forEach(el => {
  if (!map.has(el)) {
    const dup = [...arr].reduce((a, b) => (b === el ? a + 1 : a), 0);
    map.set(el, dup);
  }
});
console.log(
  `${arr.reduce((a, b) => a + +b, 0) / 10}\n${[...map.keys()].reduce((a, b) =>
    map.get(a) > map.get(b) ? a : b,
  )}`,
);