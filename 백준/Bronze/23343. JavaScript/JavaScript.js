const [A, B] = `${require('fs').readFileSync(0)}`.trim().split(' ');
console.log(A - B);