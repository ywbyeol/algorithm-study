console.log(`${require('fs').readFileSync(0)}`.trim().split('\n').reduce((a, b) => a - b));