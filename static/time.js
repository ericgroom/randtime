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

listenForTime([
  {
    predicate: date => hoursAndMinutesEqual(date, dateFromServer),
    elementFactory: celebration
  },
  {
    predicate: date => approaching(date, dateFromServer),
    elementFactory: () => shaker(timeDisplay)
  },
  {
    predicate: isPiTime,
    elementFactory: makeTextDisplay.bind(this, "\u03C0 is cool too")
  }
]);

function listenForTime(matchers) {
  var previousMatchers = matchers;
  function loop() {
    let now = new Date();

    for (let matcher of previousMatchers) {
      if (!matcher.rendered && matcher.predicate(now)) {
        matcher.rendered = matcher.elementFactory();
      } else if (matcher.rendered && !matcher.predicate(now)) {
        matcher.rendered.remove();
        matcher.rendered = null;
      }
    }
    setTimeout(loop, calculateUpdateInterval(now));
  }
  loop();
}

function celebration() {
  const el = document.createElement("p");
  el.innerText = "Woah!!!! That's right now!";
  root.appendChild(el);
  return el;
}

function shaker(el) {
  el.classList.add("shake");
  return { remove: () => el.classList.remove("shake") };
}

function makeTextDisplay(text) {
  const el = document.createElement("p");
  el.innerText = text;
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

function isPiTime(date) {
  return hours12(date) === 3 && date.getMinutes() === 14;
}
