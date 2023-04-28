const [C, K, P] = `${require('fs').readFileSync(0)}`.trim().split(' ');
let ans = 0;
for (let i = 1; i <= +C; i++) ans += +K * i + +P * i ** 2;
console.log(ans);