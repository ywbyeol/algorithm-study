const m = new Map();
`${require('fs').readFileSync(0)}`.trim().split('\n').slice(1).forEach(v => m.set(v, m.has(v) ? +m.get(v) + 1 : 1));
let a = 0;
m.forEach(v => {
  if (v > a) a = v;
});
console.log(a);