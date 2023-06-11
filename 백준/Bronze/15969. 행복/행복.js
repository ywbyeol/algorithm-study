const S = `${require('fs').readFileSync(0)}`.split('\n')[1].split(' ').map(Number);
console.log(Math.max(...S) - Math.min(...S));