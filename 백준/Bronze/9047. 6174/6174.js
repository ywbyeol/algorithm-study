const K = n => {
  let t = n.trim();
  let c = 0;
  while (t !== '6174') {
    c++;
    t = `${+[...t].sort().reverse().join('') - +[...t].sort().join('')}`.trim().padStart(4, '0');
  }
  return c;
};
console.log(`${require('fs').readFileSync(0)}`.trim().split('\n').slice(1).map(K).join('\n'));