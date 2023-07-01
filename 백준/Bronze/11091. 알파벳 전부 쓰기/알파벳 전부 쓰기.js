const ans = `${require('fs').readFileSync(0)}`
  .trim()
  .split('\n')
  .slice(1)
  .map(v => {
    const alp = new Set('abcdefghijklmnopqrstuvwxyz');
    for (const chr of v) alp.delete(chr.toLowerCase());
    return [...alp].length === 0 ? 'pangram' : `missing ${[...alp].join('')}`;
  });
console.log(ans.join('\n'));