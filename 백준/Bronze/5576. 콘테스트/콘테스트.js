const i = `${require('fs').readFileSync(0)}`.trim().split(/\s+/);
const cal = (arr, start, end) =>
  arr
    .slice(start, end)
    .sort((a, b) => +a - +b)
    .slice(-3)
    .reduce((a, b) => a + +b, 0);
console.log(cal(i, 0, 10), cal(i, 10, 20));