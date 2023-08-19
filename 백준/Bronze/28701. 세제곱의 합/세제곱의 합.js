const n = +`${require('fs').readFileSync(0)}`.trim();
const s = (n * (n + 1)) / 2;
console.log(`${s}\n${s ** 2}\n${s ** 2}`);