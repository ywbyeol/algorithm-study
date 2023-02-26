process.stdin.on('data', i => {
  const [R, S] = i.toString().trim().split('\n').map(Number);
  console.log(R * 8 + S * 3 - 28);
});