const m = new Map();
`${require('fs').readFileSync(0)}`.trim().split('\n').slice(1).forEach(v => m.set(v, (m.get(v) || 0) + 1));
console.log(Math.max(...m.values()));