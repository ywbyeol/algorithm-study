const K = n => {
  const i = (t, c) =>
    t === '6174' ? c : i(`${+[...t].sort().reverse().join('') - +[...t].sort().join('')}`.padStart(4, '0'), c + 1);
  const t = n.trim();
  return i(t, 0);
};
console.log(`${require('fs').readFileSync(0)}`.trim().split('\n').slice(1).map(K).join('\n'));