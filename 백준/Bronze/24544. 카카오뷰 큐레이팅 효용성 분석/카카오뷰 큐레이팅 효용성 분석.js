const [, C, M] = `${require('fs').readFileSync(0)}`.trim().split('\n');
const c = C.split(' ').map(Number);
console.log([c, c.map((v, i) => (+M.split(' ')[i] === 0 ? +v : 0))].map(v => v.reduce((a, b) => a + b)).join('\n'));