const n = +`${require('fs').readFileSync(0)}`.trim();
let ans = 0;
for (let i = 1; i <= Math.sqrt(n); i++) for (let j = i; i * j <= n; j++) ans++;
console.log(ans);