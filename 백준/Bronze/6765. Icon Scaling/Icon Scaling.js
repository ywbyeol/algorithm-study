const k = +`${require('fs').readFileSync(0)}`.trim();
console.log(['*x*', ' xx', '* *'].map(v => `${[...v].map(x => x.repeat(k)).join('')}\n`.repeat(k)).join('').trim());