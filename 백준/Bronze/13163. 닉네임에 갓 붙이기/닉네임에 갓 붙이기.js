const [N, ...I] = `${require('fs').readFileSync(0)}`.split('\n');
const ans = [];
for (let i = 0; i < +N; i++)
  ans.push(`god${I[i].split(' ').slice(1).join('')}`);
console.log(ans.join('\n'));