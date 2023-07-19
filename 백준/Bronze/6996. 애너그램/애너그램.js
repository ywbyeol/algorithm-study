const w = `${require('fs').readFileSync(0)}`
  .trim()
  .split('\n')
  .slice(1)
  .map(v => {
    const [a, b] = v.trim().split(' ');
    return `${a} & ${b} are ${[...a].sort().join('') === [...b].sort().join('') ? '' : 'NOT '}anagrams.`;
  });
console.log(w.join('\n'));