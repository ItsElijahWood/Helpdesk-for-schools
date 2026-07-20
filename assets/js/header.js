const p = document.getElementById("time");

const pad = n => String(n).padStart(2, '0');

function getTime() {
  const d = new Date();

  const day = pad(d.getDate());
  const month = d.toLocaleString('en-GB', { month: 'short' });
  const time = `${pad(d.getHours())}:${pad(d.getMinutes())}`;

  const s = `${day} ${month} &nbsp;${time}`;

  p.innerHTML = s;
};

getTime();
setInterval(getTime, 1000);
