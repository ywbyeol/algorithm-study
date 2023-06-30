const [N, ...T] = `${require('fs').readFileSync(0)}`.split('\n');
const ans = [];
for (let i = 0; i < +N; i++) {
  const [a, b] = T[i].split(' ');
  ans.push([...b.trim()].sort().join('') === [...a.trim()].sort().join('') ? 'Possible' : 'Impossible');
}
console.log(ans.join('\n'));