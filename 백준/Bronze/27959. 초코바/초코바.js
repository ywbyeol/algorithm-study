const [N, M] = `${require('fs').readFileSync(0)}`.split(' ');
console.log(+N * 100 >= +M ? 'Yes' : 'No');