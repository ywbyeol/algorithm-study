const r = n => (n.trim() === `${+n.trim().split('').reverse().join('')}` ? 'yes' : 'no');
console.log(`${require('fs').readFileSync(0)}`.trim().split('\n').slice(0, -1).map(r).join('\n'));