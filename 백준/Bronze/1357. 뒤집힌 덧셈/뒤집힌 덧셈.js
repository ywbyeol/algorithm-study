const r = n => +[...`${n}`].reverse().join('');
console.log(r(`${require('fs').readFileSync(0)}`.trim().split(' ').map(r).reduce((a, b) => a + b)));