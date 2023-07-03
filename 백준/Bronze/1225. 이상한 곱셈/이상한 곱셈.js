const [A, B] = `${require('fs').readFileSync(0)}`.trim().split(' ').map(v => [...v].reduce((a, b) => a + +b, 0));
console.log(A * B);