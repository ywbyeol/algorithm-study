process.stdin.on('data', i => {
  const [N, W, H, L] = i.toString().trim().split(' ').map(Number);
  console.log(Math.min(Math.floor(W / L) * Math.floor(H / L), N));
});