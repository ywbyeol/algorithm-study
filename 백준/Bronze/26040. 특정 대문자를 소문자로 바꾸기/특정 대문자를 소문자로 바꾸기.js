let [A, B] = `${require('fs').readFileSync(0)}`.trim().split('\n');
B = B.split(' ').forEach(b => (A = A.replaceAll(b, b.toLowerCase())));
console.log(A);