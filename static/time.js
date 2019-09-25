const root = document.getElementById("root");
const timeInput = document.getElementById("time-input");

const timestamp = Date.parse(timeInput.value);
const dateFromServer = new Date(timestamp);
const timeString = `${hours12(dateFromServer)}:${dateFromServer
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
celebration();
function listenForTime() {
  const now = new Date();
  if (happened) {
    happened = false;
    celebrationNode && celebrationNode.remove();
  } else if (hoursAndMinutesEqual(now, dateFromServer)) {
    happened = true;
    celebrationNode = celebration();
  }
  timeDisplay.classList.toggle("shake", approaching(now, dateFromServer));
  setTimeout(listenForTime, calculateUpdateInterval(now));
}

function celebration() {
  const el = document.createElement("canvas");
  el.classList.add("canvas");
  const context = el.getContext("2d");
  root.appendChild(el);
  return el;
}

function calculateUpdateInterval(date) {
  const secondsDelta = 59 - date.getSeconds();
  const next = secondsDelta > 0 ? secondsDelta + 1 : 60;
  return next * 1000;
}

function hoursAndMinutesEqual(a, b) {
  return hours12(a) === hours12(b) && a.getMinutes() === b.getMinutes();
}

function approaching(a, b) {
  return hours12(a) == hours12(b) && b.getMinutes() - a.getMinutes() === 1;
}

function hours12(date) {
  const hours = date.getHours() % 12;
  return hours ? hours : 12;
}
