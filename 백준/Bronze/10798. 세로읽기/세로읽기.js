const arr = `${require('fs').readFileSync(0)}`.trim().split('\n').map(v => [...v]);
let ans = '';
for (let i = 0; i < 15; i++) arr.forEach(v => (ans += v.shift() ?? ''));
console.log(ans);