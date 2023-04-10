const [a, b] = `${require('fs').readFileSync(0)}`.split(' ').map(Number);
console.log(a * (b - 1) + 1);