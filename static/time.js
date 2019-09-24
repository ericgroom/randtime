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
timeDisplay.classList.add("time", "hidden");
setTimeout(function() {
  timeDisplay.classList.remove("hidden");
}, 200);

root.appendChild(timeDisplay);

var happened = false;
var celebrationNode;
listenForTime();
function listenForTime() {
  const now = new Date();
  if (happened) {
    happened = false;
    celebrationNode && celebrationNode.remove();
  } else if (hoursAndMinutesEqual(now, dateFromServer)) {
    happened = true;
    celebrationNode = celebration();
  }
  setTimeout(listenForTime, calculateUpdateInterval(now));
}

function celebration() {
  const el = document.createElement("p");
  el.textContent = "it's happening!!!";
  root.appendChild(el);
  return el;
}

function calculateUpdateInterval(date) {
  const secondsDelta = 59 - date.getSeconds();
  const next = secondsDelta > 0 ? secondsDelta + 1 : 60;
  return next * 1000;
}

function hoursAndMinutesEqual(a, b) {
  return a.getHours() === b.getHours() && a.getMinutes() === b.getMinutes();
}
