const r = (v, i) => `Case #${i + 1}: ${v.split(' ').reverse().join(' ')}`;
console.log(`${require('fs').readFileSync(0)}`.trim().split('\n').slice(1).map((v, i) => r(v, i)).join('\n'));