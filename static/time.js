const root = document.getElementById("root");
const timeInput = document.getElementById("time-input");

const timeString = (function() {
  const timestamp = Date.parse(timeInput.value);
  const date = new Date(timestamp);
  const str = `${date.getHours()}:${date
    .getMinutes()
    .toString()
    .padStart(2, "0")}`;
  return str;
})();

const timeDisplay = document.createElement("p");
timeDisplay.textContent = timeString;
timeDisplay.classList.add("time");

root.appendChild(timeDisplay);
