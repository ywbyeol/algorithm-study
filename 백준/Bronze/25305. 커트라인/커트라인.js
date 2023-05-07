const [N, k, ...x] = `${require('fs').readFileSync(0)}`.trim().split(/\s+/g);
console.log(x.sort((a, b) => +a - +b)[+N - +k]);