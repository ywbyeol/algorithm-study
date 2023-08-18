const m = new Map();
[...`${require('fs').readFileSync(0)}`.trim().toUpperCase()].forEach(c => m.set(c, (m.get(c) || 0) + 1));
const maxChar = [...m.entries()].filter(e => e[1] === Math.max(...m.values())).map(e => e[0]);
console.log(maxChar.length === 1 ? maxChar[0] : '?');