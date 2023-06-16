console.log(
  `${require('fs').readFileSync(0)}`
    .trim()
    .toLowerCase()
    .split('\n')
    .slice(0, -1)
    .map(l => new Set(l.split('').filter(v => /[a-z]/.test(v))).size)
    .join('\n'),
);