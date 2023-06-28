const [N, M] = `${require('fs').readFileSync(0)}`.split(' ');
console.log(N.repeat(+N).slice(0, +M));