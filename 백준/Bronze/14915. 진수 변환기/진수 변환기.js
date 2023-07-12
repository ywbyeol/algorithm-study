const [m, n] = `${require('fs').readFileSync(0)}`.trim().split(' ');
console.log((+m).toString(+n).toUpperCase());