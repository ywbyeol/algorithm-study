const [D, H, W] = `${require('fs').readFileSync(0)}`.trim().split(' ');
const R = +D / Math.sqrt((+H) ** 2 + (+W) ** 2);
console.log(Math.floor(+H * R), Math.floor(+W * R));