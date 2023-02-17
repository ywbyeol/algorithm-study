process.stdin.on('data', i => {
  const [A, B] = i.toString().trim().split('\n').map(Number);
  console.log(B - A);
});