const a = `${require('fs').readFileSync(0)}`.trim().split('\n').map(v => [...v]);
let r = '';
for (let i = 0; i < 15; i++) for (const v of a) r += v.shift() ?? '';
console.log(r);