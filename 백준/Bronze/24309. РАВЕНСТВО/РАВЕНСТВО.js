process.stdin.on('data', i => {
  const [a, b, c] = i.toString().trim().split('\n').map(BigInt);
  console.log(String((b - c) / a));
});