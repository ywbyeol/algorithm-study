const [, , ZR, ZC, ...L] = `${require('fs').readFileSync(0)}`.trim().split(/\s+/);
const t = L.map(v => [...v].map(w => w.repeat(+ZC)).join(''));
console.log(t.flatMap(r => Array.from({ length: +ZR }, () => r)).join('\n'));