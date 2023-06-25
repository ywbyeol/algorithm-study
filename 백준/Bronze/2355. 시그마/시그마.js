const [a, b] = `${require('fs').readFileSync(0)}`.trim().split(' ').sort((x, y) => +x - +y);
console.log(((+a + +b) * (+b - +a + 1)) / 2);