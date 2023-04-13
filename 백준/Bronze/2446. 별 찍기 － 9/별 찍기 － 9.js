const N = +`${require('fs').readFileSync(0)}`.trim();
const a = ['*'.repeat(N * 2 - 1)];
for (let i = 1; i < N; i++) a.push(' '.repeat(i) + '*'.repeat(N * 2 - 1 - i * 2));
console.log(`${a.join('\n')}\n${a.slice(0, -1).reverse().join('\n')}`);