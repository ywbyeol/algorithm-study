const [N, , ...w] = `${require('fs').readFileSync(0)}`.trim().split(/\s+/);
console.log([...w.slice(0, +N).join('')].map(v => v.repeat(2)).join('') === w.slice(+N).join('') ? 'Eyfa' : 'Not Eyfa');