const [N, A, B] = `${require('fs').readFileSync(0)}`.trim().split(' ');
console.log(+A < +B - +N + +N ? 'Bus' : +A === +B - +N + +N ? 'Anything' : 'Subway');