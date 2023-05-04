const a = ['ABC', 'DEF', 'GHI', 'JKL', 'MNO', 'PQRS', 'TUV', 'WXYZ'];
let s = 0;
`${require('fs').readFileSync(0)}`.trim().split('')
  .forEach(x => a.forEach(v => (s += v.includes(x) ? a.indexOf(v) + 3 : 0)));
console.log(s);