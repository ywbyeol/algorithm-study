const [A, B, C] = `${require('fs').readFileSync(0)}`.trim().split(' ');
console.log(+C % 2 === 0 ? A : +A ^ +B);