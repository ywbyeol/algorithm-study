let [, ...arr] = `${require('fs').readFileSync(0)}`.trim().split('\n').map(v => (v[0] === 'P' ? 'skipped' : eval(v)));
console.log(arr.join('\n'));