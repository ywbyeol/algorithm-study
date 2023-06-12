const w = `${require('fs').readFileSync(0)}`.trim();
console.log(w === w.split('').reverse().join(''));