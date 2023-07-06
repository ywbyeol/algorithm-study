const N = `${require('fs').readFileSync(0)}`.trim();
console.log(`${N.slice(0, N.length / 2)} ${N.slice(N.length / 2)}`);