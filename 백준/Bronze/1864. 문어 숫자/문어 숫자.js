const c = { '-': 0, '\\': 1, '(': 2, '@': 3, '?': 4, '>': 5, '&': 6, '%': 7, '/': -1 };
const o = `${require('fs').readFileSync(0)}`
  .trim()
  .split(/\s+/)
  .slice(0, -1)
  .map(v => [...v].map((x, i, a) => c[x] * 8 ** (a.length - i - 1)).reduce((a, b) => a + +b, 0));
console.log(o.join('\n'));