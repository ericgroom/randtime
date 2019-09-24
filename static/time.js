const root = document.getElementById("root");
const timeInput = document.getElementById("time-input");

const timestamp = Date.parse(timeInput.value);
const dateFromServer = new Date(timestamp);
const timeString = `${dateFromServer.getHours()}:${dateFromServer
  .getMinutes()
  .toString()
  .padStart(2, "0")}`;

const timeDisplay = document.createElement("p");
timeDisplay.textContent = timeString;
timeDisplay.classList.add("time");

root.appendChild(timeDisplay);
var happened = false;
listenForTime();
function listenForTime() {
  const now = new Date();
  if (happened) {
    happened = false;
    const el = document.createElement("p");
    el.textContent = "no mas";
    root.appendChild(el);
  } else if (hoursAndMinutesEqual(now, dateFromServer)) {
    const el = document.createElement("p");
    el.textContent = "it's happening!!!";
    root.appendChild(el);
    happened = true;
  }
  setTimeout(listenForTime, calculateUpdateInterval(now));
}

function calculateUpdateInterval(date) {
  const secondsDelta = 59 - date.getSeconds();
  const next = secondsDelta > 0 ? secondsDelta + 1 : 60;
  return next * 1000;
}

function hoursAndMinutesEqual(a, b) {
  return a.getHours() === b.getHours() && a.getMinutes() === b.getMinutes();
}
