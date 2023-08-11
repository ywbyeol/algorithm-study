const [N, D] = [+`${require('fs').readFileSync(0)}`.trim(), 10];
const f = (n, d) => (n <= d ? n : f((n % d >= d / 2 ? n + d : n) - ((n % d >= d / 2 ? n + d : n) % d), d * 10));
console.log(f(N, D));