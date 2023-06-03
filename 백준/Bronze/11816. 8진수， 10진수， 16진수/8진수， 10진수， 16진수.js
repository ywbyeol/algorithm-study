const X = `${require('fs').readFileSync(0)}`.trim();
console.log(X[0] === '0' ? parseInt(X, X[1] === 'x' ? 16 : 8).toString(10) : X);