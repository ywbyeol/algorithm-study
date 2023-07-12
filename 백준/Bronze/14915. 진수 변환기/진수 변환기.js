const [m, n] = `${require('fs').readFileSync(0)}`.trim().split(' ');
console.log(parseInt(m).toString(+n).toUpperCase());