console.log("Start script");

const t0 = setTimeout(() => console.log("setTimeout3 works !!"), 3000);
const t1 = setTimeout(() => console.log("setTimeout2 works !!"), 2000);

console.log("End script");

clearTimeout(t0);

let i = 3;
let interval = setInterval(() => {
  console.log(i);
  i--;

  if (i === 0) {
    clearInterval(interval);
  }
}, 1000);
