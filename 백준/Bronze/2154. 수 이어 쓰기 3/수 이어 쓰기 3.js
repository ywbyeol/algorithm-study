const N = `${require('fs').readFileSync(0)}`.trim();
let s = '';
for (let i = 1; i <= 100000; i++) s += `${i}`;
console.log(s.indexOf(N) + 1);