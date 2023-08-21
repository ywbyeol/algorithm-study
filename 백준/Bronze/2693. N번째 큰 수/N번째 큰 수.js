const f = l =>l.split(' ').map(Number).sort((a, b) => b - a)[2];
console.log(`${require('fs').readFileSync(0)}`.trim().split('\n').slice(1).map(f).join('\n'));