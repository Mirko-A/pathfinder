const { invoke } = window.__TAURI__.tauri;

const Colors = {
  StartSquare: "green",
  EndSquare: "red",
  EmptySquare: "white",
  BlockSquare: "black",
  PathSquare: "blue",

  ButtonBackgroundInactive: "white",
  ButtonBackgroundActive: "lightblue",
};

const GridEditorMode = {
  Empty: "Empty",
  Block: "Block",
  Start: "Start",
  End: "End",
};

let mode = GridEditorMode.Empty;
let isMouseDown = false;

let emptyModeButton;
let blockModeButton;
let startModeButton;
let endModeButton;

let runButton;
let debugButton;

let gridSizeInput;
let squareSizeInput;
let createGridButton;
let gridContainer;
let grid;

let startSquare;
let endSquare;

function loadElements() {
  emptyModeButton = document.querySelector("#empty-mode-button");
  blockModeButton = document.querySelector("#block-mode-button");
  startModeButton = document.querySelector("#start-mode-button");
  endModeButton = document.querySelector("#end-mode-button");

  runButton = document.querySelector("#run-button");
  debugButton = document.querySelector("#debug-button");

  gridSizeInput = document.querySelector("#grid-size-input");
  squareSizeInput = document.querySelector("#square-size-input");
  createGridButton = document.querySelector("#create-grid-button");
  gridContainer = document.getElementById("grid-container");
  grid = document.getElementById("grid");
}

function addEventListeners() {
  addSidebarEventListeners();
  addGridEventListeners();
}

function addSidebarEventListeners() {
  const modeToButton = (mode) => {
    switch (mode) {
      case GridEditorMode.Empty:
        return emptyModeButton;
      case GridEditorMode.Block:
        return blockModeButton;
      case GridEditorMode.Start:
        return startModeButton;
      case GridEditorMode.End:
        return endModeButton;
    }
  };

  emptyModeButton.addEventListener("click", () => {
    modeToButton(mode).style.backgroundColor = Colors.ButtonBackgroundInactive;
    mode = GridEditorMode.Empty;
    emptyModeButton.style.backgroundColor = Colors.ButtonBackgroundActive;
  });
  blockModeButton.addEventListener("click", () => {
    modeToButton(mode).style.backgroundColor = Colors.ButtonBackgroundInactive;
    mode = GridEditorMode.Block;
    blockModeButton.style.backgroundColor = Colors.ButtonBackgroundActive;
  });
  startModeButton.addEventListener("click", () => {
    modeToButton(mode).style.backgroundColor = Colors.ButtonBackgroundInactive;
    mode = GridEditorMode.Start;
    startModeButton.style.backgroundColor = Colors.ButtonBackgroundActive;
  });
  endModeButton.addEventListener("click", () => {
    modeToButton(mode).style.backgroundColor = Colors.ButtonBackgroundInactive;
    mode = GridEditorMode.End;
    endModeButton.style.backgroundColor = Colors.ButtonBackgroundActive;
  });

  runButton.addEventListener("click", () => {});
  debugButton.addEventListener("click", () => {
    if (!startSquare || !endSquare) {
      alert("Please select a START and END square");
      return;
    }

    const colors = [];
    for (let i = 0; i < grid.children.length; i++) {
      colors.push(grid.children[i].style.backgroundColor);
    }

    findAndDrawShortestPath(colors, gridSizeInput.value);
  });
}

function findAndDrawShortestPath(colors, gridSize) {
  invoke("debug", {
    colors: colors,
    gridSize: gridSize,
  }).then(
    (path) => {
      for (const step of path.slice(1, path.length - 1)) {
        // row * gridSize + col
        const idx = step[0] * gridSize + step[1];

        grid.children[idx].style.backgroundColor = Colors.PathSquare;
      }
    },
    (err) => {
      console.log(err);
    },
  );
}

function addGridEventListeners() {
  createGridButton.addEventListener("click", () => {
    startSquare = null;
    endSquare = null;

    while (grid.firstChild) {
      grid.firstChild.remove();
    }

    const gridSize = gridSizeInput.value;
    const squareSize = squareSizeInput.value;

    grid.style.width = `${gridSize * squareSize}px`;
    grid.style.height = `${gridSize * squareSize}px`;
    grid.style.gridTemplateColumns = `repeat(${gridSize}, 1fr)`;
    grid.style.gridTemplateRows = `repeat(${gridSize}, 1fr)`;

    for (let i = 0; i < gridSize; i++) {
      for (let j = 0; j < gridSize; j++) {
        const gridItem = document.createElement("div");
        gridItem.classList.add("grid-item");
        gridItem.style.backgroundColor = Colors.EmptySquare;

        const events = ["mousedown", "mousemove"];

        events.forEach((event) => {
          gridItem.addEventListener(event, () => {
            if (event == "mousemove" && !isMouseDown) return;

            if (
              startSquare &&
              gridItem == startSquare &&
              mode != GridEditorMode.Start
            ) {
              startSquare.style.backgroundColor = Colors.EmptySquare;
              startSquare = null;
            }
            if (
              endSquare &&
              gridItem == endSquare &&
              mode != GridEditorMode.End
            ) {
              endSquare.style.backgroundColor = Colors.EmptySquare;
              endSquare = null;
            }

            switch (mode) {
              case GridEditorMode.Empty:
                gridItem.style.backgroundColor = Colors.EmptySquare;
                break;
              case GridEditorMode.Block:
                gridItem.style.backgroundColor = Colors.BlockSquare;
                break;
              case GridEditorMode.Start:
                if (startSquare && gridItem != startSquare) {
                  startSquare.style.backgroundColor = Colors.EmptySquare;
                }
                startSquare = gridItem;
                gridItem.style.backgroundColor = Colors.StartSquare;
                break;
              case GridEditorMode.End:
                if (endSquare && gridItem != endSquare) {
                  endSquare.style.backgroundColor = Colors.EmptySquare;
                }
                endSquare = gridItem;
                gridItem.style.backgroundColor = Colors.EndSquare;
                break;
            }
          });
        });

        grid.appendChild(gridItem);
      }
    }
  });
}

function initializeElements() {
  // Assuming default mode is Empty
  emptyModeButton.style.backgroundColor = Colors.ButtonBackgroundActive;
  // Create default grid
  createGridButton.click();

  gridContainer.addEventListener("mousedown", () => {
    isMouseDown = true;
  });
  gridContainer.addEventListener("mouseup", () => {
    isMouseDown = false;
  });
}

window.addEventListener("DOMContentLoaded", () => {
  loadElements();
  addEventListeners();
  initializeElements();
});
