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

const celebrationPredicate = date => hoursAndMinutesEqual(date, dateFromServer);
listenForTime([
  {
    predicate: celebrationPredicate,
    elementFactory: celebrationText
  },
  {
    predicate: celebrationPredicate,
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
  const el = document.createElement("canvas");
  el.classList.add("canvas");
  const context = el.getContext("2d");
  var entities = [];
  const MAX_ENTITIES = 100;
  function loop() {
    updateCanvasSize(context.canvas);
    // prune
    entities = entities.filter(e => !isOutOfBounds(e));
    // render
    entities.forEach(e => e.render(context));
    frameId = requestAnimationFrame(loop);
  }
  var frameId = requestAnimationFrame(loop);

  function physicsLoop() {
    entities.forEach(e => e.tick());
    if (entities.length < MAX_ENTITIES) {
      entities = entities.concat(emitter());
    }
  }
  var intervalId = setInterval(physicsLoop, 10);
  root.appendChild(el);
  return {
    remove: () => {
      el.remove();
      cancelAnimationFrame(frameId);
      clearInterval(intervalId);
    }
  };
}

function celebrationText() {
  const el = document.createElement("p");
  el.innerText = "Woah!!!! That's right now!\nMake a wish!";
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

function randomNumber(min, max) {
  return Math.random() * (max - min) + min;
}

function randomInteger(min, max) {
  return Math.floor(Math.random() * (max - min + 1)) + min;
}

function choice(options) {
  return options[randomInteger(0, options.length - 1)];
}

// Physics

function updateCanvasSize(canvas) {
  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;
}

function emitter() {
  const min = 1;
  const max = 5;
  const amount = randomInteger(min, max);

  const entities = [];
  for (let i = 0; i < amount; i++) {
    const entity = makeDot();
    entities.push(entity);
  }
  return entities;
}

function makeDot() {
  const xMin = timeDisplay.offsetLeft;
  const xMax = timeDisplay.offsetWidth + xMin;
  const xMid = (xMin + xMax) / 2;
  const y = timeDisplay.offsetTop + timeDisplay.offsetHeight * 0.15;
  const x = randomNumber(xMin, xMax);
  const vy = randomNumber(-5, -20);
  const vx = x < xMid ? randomNumber(-10, -5) : randomInteger(5, 10);
  const color = choice(["#a864fd", "#29cdff", "#77dd77", "#ff718d", "#fef284"]);
  const shape = new Shape(x, y, 10, 10, color);
  const physicsShape = new PhysicsObject(shape, vx, vy, 0, 0.5);
  physicsShape.render = function(context) {
    context.beginPath();
    context.ellipse(this.x, this.y, this.w, this.h, 0, 0, Math.PI * 2);
    context.closePath();
    context.fillStyle = this.color;
    context.fill();
  };
  return physicsShape;
}

function Shape(x, y, w, h, color) {
  this.x = x;
  this.y = y;
  this.w = w;
  this.h = h;
  this.color = color;
}

Shape.prototype.render = function(context) {
  context.rect(this.x, this.y, this.w, this.h);
  context.fillStyle = this.color;
  context.fill();
};

function PhysicsObject(shape, vix, viy, aix, aiy) {
  this.x = shape.x;
  this.y = shape.y;
  this.w = shape.w;
  this.h = shape.h;
  this.color = shape.color;
  this.vx = vix;
  this.vy = viy;
  this.ax = aix;
  this.ay = aiy;
}

PhysicsObject.prototype.tick = function() {
  this.vx += this.ax;
  this.vy += this.ay;

  this.x += this.vx;
  this.y += this.vy;
};

function isOutOfBounds(shape) {
  return (
    shape.x < 0 ||
    shape.x > window.outerWidth ||
    shape.y < 0 ||
    shape.y > window.outerHeight
  );
}
