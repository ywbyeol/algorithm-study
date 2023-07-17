let [, ...l] = `${require('fs').readFileSync(0)}`.trim().split('\n');
l = l.map(v => {
  const [a, n] = v.split('-');
  return Math.abs([...a].map((c, i) => (c.charCodeAt(0) - 65) * 26 ** (2 - i)).reduce((x, y) => x + +y, 0) - +n) < 101
    ? 'nice'
    : 'not nice';
});
console.log(l.join('\n'));