const N = +`${require('fs').readFileSync('/dev/stdin')}`.trim();
const ans = [];
for (let i = 1; i <= N; i++) ans.push(`${' '.repeat(N - i) + '*'.repeat(i)}`);
console.log([...ans, ...ans.reverse().slice(1)].join('\n'));