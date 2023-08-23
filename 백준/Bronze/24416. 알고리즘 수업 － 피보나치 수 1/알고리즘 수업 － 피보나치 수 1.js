const n = +`${require('fs').readFileSync(0)}`.trim();
const f = [0, 1, 1];
for (let i = 3; i <= n; i++) f[i] = f[i - 1] + f[i - 2];
console.log(`${f[n]} ${n - 2}`);