const N = +`${require('fs').readFileSync('/dev/stdin')}`.trim();
const a = [];
for (let i = 0; i < N; i++)
  a.push(`${' '.repeat(N - i - 1)}${'*'.repeat(i * 2 + 1)}`);
console.log([...a, ...a.slice(0, -1).reverse()].join(`\n`));