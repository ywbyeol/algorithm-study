const r = l => l.split(' ').map(v => [...v].reverse().join('')).join(' ');
console.log(`${require('fs').readFileSync(0)}`.trim().split('\n').slice(1).map(r).join('\n'));